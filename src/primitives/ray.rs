use math::vectors::Vector;
use primitives::sphere::Sphere;

#[derive(Debug,Clone,Copy)]
pub struct Ray{
    pub origin: Vector,
    pub direction: Vector,
}
#[derive(Debug,Clone,Copy)]
pub struct Intersection{
    pub hit_p: Vector,
    pub n: Vector,

}


impl Ray{ 
    
    pub fn intersection(self,sphere: Sphere)-> Option<Intersection> {
            //base formula = |O +tD -C|2 - R2 = 0
            // which goes to greating a quadratic formula
            // use base quadtratic form math to solve

            // getting base value for quadratic function
            let l = self.origin - sphere.center;
            let a = self.direction.dot_product(self.direction);
            let b = 2.0 * self.direction.dot_product(l);
            let c = l.dot_product(l) - sphere.radius.powf(2.0);
            let mut pt0 = 0.0;
            let mut pt1 = 0.0;
            //quadratic function
            
            let discriminant = b.powf(2.0) - 4.0 * a * c;
            
            if discriminant > 0.0 {
                pt0 = (-b + discriminant.sqrt()) / (2.0 * a);
                pt1 = (-b - discriminant.sqrt()) / (2.0 * a);
            } else if discriminant == 0.0 {
                pt0 = -0.5 * ( b/a);
            } else if discriminant < 0.0 {
                return None;
            }
            
            // If points are both infront of you, take closet
            // if pt0 is infront take pt0
            // if pt1 is infront take pt1
            // otherwise both are behind
            // this checks to see where the poiints are located
            // and give you closet point

            let t = {
                if pt0 > 0.0 && pt1 > 0.0 {
                   if pt0 > pt1 { pt1 } else { pt0 }
                }
                else if pt0 > 0.0 { pt0 }
                else if pt1 > 0.0 { pt1 }
                else { return None; }
            };
        
            let phit = self.origin + ( self.direction * t);
            let intersect = Intersection{
                            hit_p : phit,
                            n : (phit - sphere.center).normalize()};
            Some(intersect)
    }
}
