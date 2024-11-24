use crate::colour::Colour;
use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub colour: Colour,
}

impl Material {
    pub fn default() -> Self {
        Material {
            colour: Colour::white(),
        }
    }
}

const INTERSECTION_EPSILON: f64 = 1e-4;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntersectionResult {
    No,
    One(f64),
    Two(f64, f64),
}

pub trait Entity {
    fn intersection(&self, ray: Vector) -> IntersectionResult;
    fn material(&self) -> Material;
    fn normal(&self, position: Vector) -> Vector;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    position: Vector,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(position: Vector, radius: f64, colour: Colour) -> Self {
        Self {
            position,
            radius,
            material: Material { colour },
        }
    }
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
            // Sphere is behind camera
            if t > 0. {
                return IntersectionResult::One(t);
            } else {
                return IntersectionResult::No;
            }
        } else {
            let t1 = (-b + delta.sqrt()) / (2. * a);
            let t2 = (-b - delta.sqrt()) / (2. * a);
            // We're inside the sphere
            if t1 > 0. {
                return IntersectionResult::Two(t1, t2);
            } else {
                return IntersectionResult::No;
            }
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, at: Vector) -> Vector {
        (at - self.position).normalised()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RaycastResult {
    pub hit: bool,
    pub position: Vector,
    pub normal: Vector,
    pub material: Material,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    pub position: Vector,
    pub intensity: f64,
}

impl Light {
    pub fn new(position: Vector, intensity: f64) -> Self {
        Self {
            position,
            intensity,
        }
    }
    pub fn default() -> Self {
        Self::new(Vector::new(-1., 1., 1.), 1.)
    }
}

pub struct World {
    pub entities: Vec<Box<dyn Entity>>,
    pub light: Light,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            light: Light::default(),
        }
    }

    pub fn find_nearest(&self, ray: Vector) -> RaycastResult {
        let mut dist = f64::INFINITY;
        let mut closest_entity: &Box<dyn Entity> = &self.entities[0];

        for entity in &self.entities {
            match entity.intersection(ray) {
                IntersectionResult::No => {}
                IntersectionResult::One(t) => {
                    if t < dist {
                        dist = t;
                        closest_entity = entity;
                    }
                }
                IntersectionResult::Two(t1, _) => {
                    if t1 < dist {
                        dist = t1;
                        closest_entity = entity;
                    }
                }
            }
        }

        let hit = dist < f64::INFINITY;

        let mut result = RaycastResult {
            hit: false,
            position: Vector::zero(),
            normal: Vector::zero(),
            material: Material::default(),
        };

        if hit {
            let position = ray * dist;
            result.hit = true;
            result.position = position;
            result.normal = closest_entity.normal(ray);
            result.material = closest_entity.material();
        }

        return result;
    }
}
