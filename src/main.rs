mod colour;
mod image;
mod raytrace;
mod vector;
mod world;

use std::env;
use std::error::Error;
use std::io::Read;
use std::{fs::File, io::Write};

use image::BMPImage;
use world::World;

const USAGE: &str = "USAGE: ./raytrace [world_spec] [output_file]";

fn main() {
    let args = env::args();

    let mut world_spec_filename = None;
    let mut output_filename = None;
    let mut num_args = 0;
    for (i, arg) in args.enumerate() {
        num_args += 1;

        // positional args
        match i {
            1 => {
                world_spec_filename = Some(arg);
            }
            2 => {
                output_filename = Some(arg);
            }
            _ => {}
        }
    }

    if num_args < 3 {
        println!("{}", USAGE);
        return;
    }

    match open_and_parse_toml(world_spec_filename.unwrap().as_str()) {
        Ok(world) => {
            let image = raytrace::render(&world, 400, 400);

            let bmpimage = BMPImage::from(image);

            File::create(output_filename.unwrap())
                .unwrap()
                .write(bmpimage.as_bytes().as_slice())
                .unwrap();
        }
        Err(e) => {
            eprintln!("Error: {:}", e);
        }
    }
}

fn open_and_parse_toml(filename: &str) -> Result<World, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let table = buf.parse::<toml::Table>()?;
    return Ok(World::from_toml(&table));
}
