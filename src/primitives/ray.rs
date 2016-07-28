use math::vectors::Vector;
use primitives::sphere::Sphere;

#[derive(Debug,Clone,Copy)]

// Rays can be described as O + D *t
// O = orgin or ray
// D = direction of ray
// t = distance from origin to intersection point

pub struct Ray{
    pub origin: Vector,
    pub direction: Vector,
}
#[derive(Debug,Clone,Copy)]
pub struct Intersection{
    pub hit_p: Vector,
    pub n: Vector,

}

