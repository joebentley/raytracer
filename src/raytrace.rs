// Resources:
// Intersection maths: https://upload.wikimedia.org/wikipedia/commons/9/95/Ray_Tracing_Illustration_First_Bounce.png
// Ray tracing algo: https://en.wikipedia.org/wiki/Path_tracing#Algorithm
// Basic aligned camera: https://computergraphics.stackexchange.com/questions/8479/how-to-calculate-ray

use core::f64;

use crate::colour::Colour;
use crate::image::Image;
use crate::vector::Vector;
use crate::world::World;

pub fn trace_path_no_recurse(world: &World, ray: Vector) -> Option<Colour> {
    let result = world.find_nearest(ray);

    if !result.hit {
        return None;
    }

    let material = result.material;
    // TODO: material emittance

    let light = world.light;
    let hit_to_light = (light.position - result.position).normalised();
    let cos_angle = hit_to_light.dot(&result.normal) as f32;

    Some(material.colour * cos_angle)
}

pub fn render(world: &World, width: u16, height: u16) -> Image {
    let mut image = Image::new(width, height);

    let theta_fov = 90. * f64::consts::PI / 180.;
    let d = 1. / (theta_fov / 2.).tan();

    let aspect_ratio = width as f64 / height as f64;

    for y in 0..height {
        let p_y = y as f64 + 0.5;
        for x in 0..width {
            let p_x = x as f64 + 0.5;

            // Ray from camera
            let ray = Vector::new(
                aspect_ratio * 2. * p_x / width as f64 - 1.,
                2. * p_y / height as f64 - 1.,
                d,
            );

            let mut c = Colour::black();
            if let Some(colour) = trace_path_no_recurse(world, ray) {
                c = colour;
            }
            image.put_pixel(x, y, c.as_rgb24());
        }
    }

    image
}
