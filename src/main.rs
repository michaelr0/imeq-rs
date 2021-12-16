use clap::{load_yaml, App};

fn main() {
    // Init APP/CLI
    let config = load_yaml!("main.yml");
    let app = App::from_yaml(config);
    let cli = app.get_matches();

    // Get image paths from CLI
    let images = imeq::get_image_names(&cli);

    // Check if images use the same path
    imeq::check_image_same_path(&images);

    // Open images and read bytes
    let images = imeq::open_images(images);

    // Generate hashes
    let hashes = imeq::get_hashes(&images);

    // Check if images have matching hashes
    imeq::check_image_hashes(hashes);

    // Compare images dimensions and pixels
    imeq::compare_image_as_images(&images);

    // Everything else has led to this moment!
    // The images appear to match!
    println!("Images are a match");
}
