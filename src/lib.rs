use clap::ArgMatches;
use image::{DynamicImage, GenericImageView, Pixels};
use rayon::prelude::*;
use ring::digest::{self};
use std::{fs::File, io::Read, process::exit};

/// Check if images use the same path
pub fn get_image_names(cli: &ArgMatches) -> Vec<String> {
    vec![
        cli.value_of("IMAGE_1")
            .expect("first image is required")
            .to_string(),
        cli.value_of("IMAGE_2")
            .expect("second image is required")
            .to_string(),
    ]
}

/// Open images and read bytes
pub fn open_images(images: Vec<String>) -> Vec<Vec<u8>> {
    images
        .par_iter()
        .map(|i| {
            let mut buf = Vec::new();

            File::open(i)
                .expect("Unable to open file")
                .read_to_end(&mut buf)
                .unwrap();

            buf
        })
        .collect::<Vec<Vec<u8>>>()
}

/// Generate hashes
pub fn get_hashes(images: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    images
        .par_iter()
        .map(|i| digest::digest(&digest::SHA512, i).as_ref().to_owned())
        .collect::<Vec<Vec<u8>>>()
}

/// Check if images use the same path
pub fn check_image_same_path(images: &Vec<String>) {
    if images.first() == images.last() {
        println!("Images are the same file");
        exit(0);
    }
}

/// Check if images have matching hashes
pub fn check_image_hashes(hashes: Vec<Vec<u8>>) {
    if hashes.first().eq(&hashes.last()) {
        println!("Images are the same file");
        exit(0);
    }
}

/// Compare images dimensions and pixels
pub fn compare_image_as_images(images: &Vec<Vec<u8>>) {
    let images = images
        .par_iter()
        .map(|i| image::load_from_memory(i).expect("Could not read image"))
        .collect::<Vec<DynamicImage>>();

    // Compare images dimensions
    if !image_dimensions_match(&images) {
        println!("Images aren't the same dimensions");
        exit(0);
    }

    let images = images
        .par_iter()
        .map(|i| i.pixels())
        .collect::<Vec<Pixels<DynamicImage>>>();

    let image1 = images.first().unwrap().to_owned();
    let image2 = images.last().unwrap().to_owned();

    // Compare images pixels
    if image1.ne(image2) {
        println!("Images are not a match");
        exit(0);
    }
}

/// Compare images dimensions
pub fn image_dimensions_match(images: &Vec<DynamicImage>) -> bool {
    let image1_dimensions = images.first().unwrap().dimensions();
    let image2_dimensions = images.last().unwrap().dimensions();

    image1_dimensions.eq(&image2_dimensions)
}
