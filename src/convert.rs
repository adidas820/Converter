pub fn convert_img(src: &str, dest: &str) {
    let img = image::open(src).unwrap();
    img.save(dest).unwrap();
}
