fn main() {
    // path args
    let args: Vec<String> = std::env::args().collect();
    let image_path = &args[1];
    let cwd = std::env::current_dir().expect("Failed to get current directory");
    let path = cwd.join(image_path);

    // read image
    let reader = std::io::BufReader::new(std::fs::File::open(path).expect("Failed to open file"));
    let img_buffer = image::io::Reader::new(reader)
        .with_guessed_format()
        .expect("Failed to guess image format");

    let img_format = img_buffer.format().expect("Failed to get image format");
    let img = img_buffer.decode().expect("Failed to decode image");
    let img_type = img.color();

    // get image dimensions
    let (width, height) = (img.width(), img.height());

    // get rgba pixels
    let (rgb_buffer, img_ext) = match img_format {
        image::ImageFormat::Jpeg => (img.to_rgb8().into_raw(), "jpg"),
        image::ImageFormat::Png => (img.to_rgba8().into_raw(), "png"),
        _ => panic!("Unsupported image format"),
    };

    // generate random key
    let key = rgb_buffer
        .iter()
        .map(|_| rand::random::<u8>())
        .collect::<Vec<u8>>();

    // encrypt image
    let encrypted = rgb_buffer
        .iter()
        .zip(key.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<u8>>();

    // save encrypted image
    image::save_buffer_with_format(
        format!("encrypted.{}", img_ext),
        &encrypted,
        width,
        height,
        img_type,
        img_format,
    )
    .expect("Failed to save encrypted image");

    // decrypt image
    let decrypted: Vec<u8> = encrypted
        .iter()
        .zip(key.iter())
        .map(|(a, b)| a ^ b)
        .collect();

    // save decrypted image
    image::save_buffer_with_format(
        format!("decrypted.{}", img_ext),
        &decrypted,
        width,
        height,
        img_type,
        img_format,
    )
    .expect("Failed to save decrypted image");
}
