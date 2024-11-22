mod image;

use std::{fs::File, io::Write};

use image::{BMPImage, Image};

fn main() {
    let image = Image::new(400, 400);
    let bmpimage = BMPImage::from(image);

    File::create("test.bmp")
        .unwrap()
        .write(bmpimage.as_bytes().as_slice())
        .unwrap();
}
