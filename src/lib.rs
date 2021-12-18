use image::{DynamicImage, GenericImageView, Pixels};
use rayon::prelude::*;
use ring::digest::{self};
use std::{collections::HashMap, fs::File, io::Read};

pub struct Compare<'a> {
    images: [String; 2],
    checks: HashMap<&'a str, bool>,
}

impl Compare<'_> {
    pub fn new(image_1: String, image_2: String) -> Self {
        let mut compare = Compare {
            images: [image_1, image_2],
            checks: HashMap::new(),
        };

        compare.checks.insert("images_have_same_path", false);
        compare.checks.insert("image_hashes_match", false);
        compare.checks.insert("image_dimensions_match", false);
        compare.checks.insert("image_pixels_match", false);

        compare
    }

    pub fn enable_check(&mut self, check: &'static str) -> &mut Self {
        self.checks.insert(check, true);

        self
    }

    pub fn enable_check_images_have_same_path(&mut self) -> &mut Self {
        self.enable_check("images_have_same_path")
    }

    pub fn enable_check_image_hashes_match(&mut self) -> &mut Self {
        self.enable_check("image_hashes_match")
    }

    pub fn enable_check_images_dimensions_match(&mut self) -> &mut Self {
        self.enable_check("image_dimensions_match")
    }

    pub fn enable_check_images_pixels_match(&mut self) -> &mut Self {
        self.enable_check("image_pixels_match")
    }

    fn is_check_enabled(&self, check: &'static str) -> bool {
        *self.checks.get(check).unwrap_or(&false)
    }

    pub fn are_match(&self) -> bool {
        if self.is_check_enabled("images_have_same_path")
            && self.images.first() == self.images.last()
        {
            return true;
        }

        let images = self
            .images
            .par_iter()
            .map(|i| {
                let mut buf = Vec::new();

                File::open(i)
                    .expect("Unable to open file")
                    .read_to_end(&mut buf)
                    .unwrap();

                buf
            })
            .collect::<Vec<Vec<u8>>>();

        if self.is_check_enabled("image_hashes_match") {
            let hashes = images
                .par_iter()
                .map(|i| digest::digest(&digest::SHA512, i).as_ref().to_owned())
                .collect::<Vec<Vec<u8>>>();

            if hashes.first().eq(&hashes.last()) {
                return true;
            }
        }

        let images = images
            .par_iter()
            .map(|i| image::load_from_memory(i).expect("Could not read image"))
            .collect::<Vec<DynamicImage>>();

        if self.is_check_enabled("image_dimensions_match") {
            let image1_dimensions = images.first().unwrap().dimensions();
            let image2_dimensions = images.last().unwrap().dimensions();

            if image1_dimensions.ne(&image2_dimensions) {
                return false;
            }
        }

        if self.is_check_enabled("image_pixels_match") {
            let images = images
                .par_iter()
                .map(|i| i.pixels())
                .collect::<Vec<Pixels<DynamicImage>>>();

            let image1 = images.first().unwrap().to_owned();
            let image2 = images.last().unwrap().to_owned();

            if image1.eq(image2) {
                return true;
            }
        }

        false
    }

    pub fn arent_match(&self) -> bool {
        !self.are_match()
    }
}
