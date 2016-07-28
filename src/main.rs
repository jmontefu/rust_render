extern crate image;

mod math;
mod primitives;
use math::vectors::Vector;
use primitives::sphere::Sphere;
//use primitives::sphere::Intersection;
use primitives::triangle::Triangle;
use primitives::ray::Intersection;
use primitives::ray::Ray;

use std::fs::File;
use std::path::Path;


fn main() {
    let sphere_center = Vector::new(0.0,0.0,0.0);
    let sphere = Sphere{radius: 0.5,
                        center: sphere_center};
    
    let camera_origin = Vector::new(0.0,0.0,-15.0);
    let triangle = Triangle{
                            v0: Vector::new(-0.5,0.0,0.0),
                            v1: Vector::new(0.5,0.0,0.0),
                            v2: Vector::new(0.0,0.750,0.0),};
                            

    let imgx = 400;
    let imgy = 400;
    let mut imgbuf = image::ImageBuffer::new(imgx,imgy);

    for (x,y,pixel) in imgbuf.enumerate_pixels_mut(){
        let spx = 2.0 * ((x as f32 + 0.5) / imgx as f32) - 1.0;
        let spy = 1.0 - 2.0 * ((y as f32 + 0.5) / imgy as f32) ;
        
        //let ray_direction = (Vector::new(spx,spy,0.0) - camera_origin);

        let ray = Ray{origin: Vector::new(spx,spy,-10.0),
                      direction: Vector::new(0.0,0.0,-1.0)};
        //let intersect = Sphere::intersection(sphere,ray);
        let intersect = Triangle::geo_intersection(triangle,ray);

        match intersect {
            Some(int) => {

            // multiply by half of 255 take it makes 
            // the nromal range is -1 to 1 
            // by multiplying 127.5 it now goes between -127.5 and 127.5
            // then adding 127.5 brings the values between 0 and 255

                *pixel = image::Rgba([(int.n.x * 127.5 + 127.5) as u8,
                                      (int.n.y * 127.5 + 127.5) as u8,
                                      (int.n.z * 127.5 + 127.5) as u8,
                                      255]);
                                      },
            None => *pixel = image::Rgba([0,0,0,255]),

        }
    }
    

    let ref mut fout = File::create(&Path::new("test.png")).unwrap();
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PNG); 
}
