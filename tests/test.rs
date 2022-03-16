#[test]
pub fn are_same_path() {
    let image_1 = "images/baseline.jpeg".to_string();
    let image_2 = "images/baseline.jpeg".to_string();

    let images_match = imeq::Compare::new(image_1, image_2)
        .enable_check_images_have_same_path()
        .are_match();

    assert!(images_match);
}

#[test]
pub fn are_match() {
    let image_1 = "images/baseline.jpeg".to_string();
    let image_2 = "images/baseline_by_another_name.jpeg".to_string();

    let images_match = imeq::Compare::new(image_1, image_2)
        .enable_check_image_hashes_match()
        .enable_check_images_dimensions_match()
        .enable_check_images_pixels_match()
        .are_match();

    assert!(images_match);
}

#[test]
pub fn are_not_match1() {
    let image_1 = "images/baseline.jpeg".to_string();
    let image_2 = "images/modified.jpeg".to_string();

    let images_match = imeq::Compare::new(image_1, image_2)
        .enable_check_images_have_same_path()
        .enable_check_image_hashes_match()
        .enable_check_images_dimensions_match()
        .enable_check_images_pixels_match()
        .are_match();

    assert!(!images_match);
}

#[test]
pub fn are_not_match2() {
    let image_1 = "images/baseline.jpeg".to_string();
    let image_2 = "images/flipped.jpeg".to_string();

    let images_match = imeq::Compare::new(image_1, image_2)
        .enable_check_images_have_same_path()
        .enable_check_image_hashes_match()
        .enable_check_images_dimensions_match()
        .enable_check_images_pixels_match()
        .are_match();

    assert!(!images_match);
}
