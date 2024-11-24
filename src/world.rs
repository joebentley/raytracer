use crate::vector::Vector;

const INTERSECTION_EPSILON: f64 = 1e-4;
pub enum IntersectionResult {
    No,
    One(f64),
    Two(f64, f64),
}

trait Entity {
    fn intersection(&self, ray: Vector) -> IntersectionResult;
}

pub struct Sphere {
    position: Vector,
    radius: f64,
}

impl Entity for Sphere {
    fn intersection(&self, normalised_ray: Vector) -> IntersectionResult {
        // Solving this https://upload.wikimedia.org/wikipedia/commons/9/95/Ray_Tracing_Illustration_First_Bounce.png

        // d^2 t^2 + 2(o - c).d t + (o - c)^2 - r^2 = 0
        // a = d^2, b = 2(o - c).d, c = (o - c)^2 - r^2
        // t = (-b ± sqrt(b^2 - 4ac)) / 2a

        // Camera is at origin in our system
        let o = Vector::zero();
        let d = normalised_ray;
        let c = self.position;
        let r = self.radius;

        let a = d.dot(&d);
        let b = 2. * (o - c).dot(&d);
        let c = (o - c).abs_squared() - r * r;

        let delta = b * b - 4. * a * c;
        if delta < 0. {
            return IntersectionResult::No;
        } else if delta.abs() < INTERSECTION_EPSILON {
            let t = -b / (2. * a);
            return IntersectionResult::One(t);
        } else {
            let t1 = (-b + delta.sqrt()) / (2. * a);
            let t2 = (-b - delta.sqrt()) / (2. * a);
            return IntersectionResult::Two(t1, t2);
        }
    }
}

pub struct World {
    entities: Vec<Box<dyn Entity>>,
}
