use clap::{load_yaml, App};

fn main() {
    let config = load_yaml!("main.yml");
    let app = App::from_yaml(config);
    let cli = app.get_matches();

    let image_1 = cli
        .value_of("IMAGE_1")
        .expect("first image is required")
        .to_string();

    let image_2 = cli
        .value_of("IMAGE_2")
        .expect("second image is required")
        .to_string();

    let images_match = imeq::Compare::new(image_1, image_2)
        .enable_check_images_have_same_path()
        .enable_check_image_hashes_match()
        .enable_check_images_dimensions_match()
        .enable_check_images_pixels_match()
        .are_match();

    if images_match {
        println!("Images are a match");
    } else {
        println!("Images are not a match");
    }
}
