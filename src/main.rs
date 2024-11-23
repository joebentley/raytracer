mod colour;
mod image;
mod vector;

use std::{fs::File, io::Write};

use colour::Colour;
use image::{BMPImage, Image};

fn main() {
    let mut image = Image::new(400, 400);

    for y in 0..image.height as usize {
        let g = y as f32 / image.height as f32;
        for x in 0..image.width as usize {
            let r = x as f32 / image.width as f32;
            image.put_pixel(x, y, Colour::new(r, g, 0.).as_rgb24());
        }
    }

    let bmpimage = BMPImage::from(image);

    File::create("test.bmp")
        .unwrap()
        .write(bmpimage.as_bytes().as_slice())
        .unwrap();
}
