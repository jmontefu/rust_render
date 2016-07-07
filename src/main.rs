// HOME WORK

//Create Ray Struct
// Ray has starting point, direction, and
// a function called intersect.
// intersect function takes ray and sphere
// tels you if ray hit sphere and where


//extern crate rust_training;
//use rust_training::vectors::maths;


// mod : imports module vectors
// use : use "Vector" instead of typing "vectors::maths::Vector"
// inside of scripts: same as " import xx as xx" in python


mod math;
mod geometry;
use math::vectors::Vector;
use std::ops::Sub;
use geometry::sphere::Sphere;
use geometry::sphere::Ray;

fn main() {
    let sphere_center = Vector::new(4.0,1.0,31.0);
    let ray_orig = Vector::new(2.2,1.0,30.0);
    let direction = Vector::new(1.0,0.0,0.0);

    let sphere = Sphere{radius: 1.2, 
                        center: sphere_center};

    let ray = Ray{origin: ray_orig,
                  direction: direction};

    let intersect = Sphere::intersection(sphere,ray);

    println!("test: {:?}",intersect);
}
