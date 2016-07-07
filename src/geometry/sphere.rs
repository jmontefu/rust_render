// modules for sphere

use math::vectors::Vector;

/// HOMEWORK
// single ray struct
// single sphere strcu
// Single Vector struct
// Intersection Struct or method
//     on sphere

// look up enum //

#[derive(Debug,Clone,Copy)]
pub struct Sphere{
    pub radius: f32,
    pub center: Vector,
}


// rays can be expressed by
//  O +t D : O = orgin, D = Direction, t= param of function
// if t is positive, point is infront of O
// if t is negative, point is behind O
// if t is 0, point is at O
#[derive(Debug,Clone,Copy)]
pub struct Ray{
    pub origin: Vector,
    pub direction: Vector,
}


impl Sphere {    
    
    pub fn intersection(self,ray: Ray)-> Vector {
            //base formula = |O +tD -C|2 - R2 = 0
            // which goes to greating a quadratic formula
            // use base quadtratic form math to solve

            // getting base value for quadratic function
            let l = ray.origin - self.center;
            let a = ray.direction.dot_product(ray.direction);
            let b = ray.direction.dot_product(l);
            let c = l.dot_product(l) - self.radius.powf(2.0);
            let mut pt0 = 0.0;
            let mut pt1 = 0.0;
            //quadratic function
            
            let discriminant = b.powf(2.0) - 4.0 * a * c;
            
            if discriminant > 0.0 {
                pt0 = (-b + discriminant.sqrt()) / (2.0 * a);
                pt1 = (-b - discriminant.sqrt()) / (2.0 * a);
            } else if discriminant == 0.0 {
                pt0 = -0.5 * ( b/a) ;
            } else if discriminant < 0.0 {
                pt0 = 0.11; 
            }

        
            let phit = ray.origin + ( ray.direction * pt0);

            return phit;
    }

}


