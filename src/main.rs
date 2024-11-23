mod image;

use std::{fs::File, io::Write};

use image::{BMPImage, Image};

fn main() {
    let mut image = Image::new(400, 400);

    image.fill_with(0xFF, 0, 0);

    let bmpimage = BMPImage::from(image);

    File::create("test.bmp")
        .unwrap()
        .write(bmpimage.as_bytes().as_slice())
        .unwrap();
}
