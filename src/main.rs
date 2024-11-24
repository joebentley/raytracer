mod colour;
mod image;
mod raytrace;
mod vector;
mod world;

use std::{fs::File, io::Write};

use colour::Colour;
use image::{BMPImage, Image};
use vector::Vector;
use world::{Sphere, World};

fn main() {
    let mut world = World::new();
    let sphere = Box::new(Sphere::new(
        Vector::new(0., 0., 1.),
        1.,
        Colour::new(1., 0., 0.),
    ));
    world.entities.push(sphere);

    let image = raytrace::render(&world, 400, 400);

    let bmpimage = BMPImage::from(image);

    File::create("test.bmp")
        .unwrap()
        .write(bmpimage.as_bytes().as_slice())
        .unwrap();
}
