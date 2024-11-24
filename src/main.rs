mod colour;
mod image;
mod raytrace;
mod vector;
mod world;

use std::env;
use std::{fs::File, io::Write};

use colour::Colour;
use image::BMPImage;
use vector::Vector;
use world::{Light, Sphere, World};

const USAGE: &str = "USAGE: ./raytrace [output_file]";

fn main() {
    let args = env::args();

    let mut filename = None;
    let mut num_args = 0;
    for (i, arg) in args.enumerate() {
        num_args += 1;
        if i == 1 {
            filename = Some(arg);
        }
    }

    if num_args == 1 {
        println!("{}", USAGE);
        return;
    }

    let mut world = World::new();
    world.light = Light::new(Vector::new(-1., -4., -1.), 1.1);
    let sphere = Box::new(Sphere::new(
        Vector::new(0., 0., 4.),
        2.,
        Colour::new(1., 0., 0.),
    ));
    world.entities.push(sphere);

    let image = raytrace::render(&world, Colour::white(), 400, 400);

    let bmpimage = BMPImage::from(image);

    File::create(filename.unwrap())
        .unwrap()
        .write(bmpimage.as_bytes().as_slice())
        .unwrap();
}
