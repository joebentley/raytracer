mod colour;
mod image;

use std::{fs::File, io::Write};

use colour::Colour;
use image::{BMPImage, Image};

fn main() {
    let mut image = Image::new(400, 400);

    for y in 0..image.height as usize {
        for x in 0..image.width as usize {
            image.put_pixel(x, y, Colour::new(1., 0., 0.).as_rgb24());
        }
    }

    let bmpimage = BMPImage::from(image);

    File::create("test.bmp")
        .unwrap()
        .write(bmpimage.as_bytes().as_slice())
        .unwrap();
}
