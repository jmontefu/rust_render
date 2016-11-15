use math::vectors::Vector;
use primitives::ray::Ray;
use primitives::ray::Intersection;


#[derive(Debug,Clone,Copy)]
pub struct Triangle{
    pub v0: Vector,
    pub v1: Vector,
    pub v2: Vector,
}




impl Triangle{


    pub fn new(v0: Vector,v1: Vector,v2: Vector) -> Triangle{
        Triangle{
            v0: v0,
            v1: v0,
            v2: v1,
        }


    }
    pub fn normal(self)-> Vector{
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let n = v0v1.cross_product(v0v2).normalize();
        n
    }

    // Intersection equations
    // Phit = O + tR 
    // O = ray origin, R = ray Direction , t = distance

    pub fn intersection(self,ray: Ray) -> Option<Intersection>{
         
        let n = self.normal();
        // finds distance from origin to v0
        // to find didstance from plan of the nomal
        // d = distance from orgin to plane of the normal
        // everytime you dot product with a normal youre getting
        // the distacne from the point your specify to the plane

        let d = n.dot_product(self.v0);
       // this tesll how many ray distances will i t take to get to the 
       // plane of the normal
        let t = ((n.dot_product(ray.origin) + d) / n.dot_product(ray.direction));
        
        if t <= 0.0 {
            return None;
            }

        let phit = ray.origin + t * ray.direction;

        let v0v1 = self.v1 - self.v0;
        let v1v2 = self.v2 - self.v1;
        let v2v0 = self.v0 - self.v2;

        let c0 = phit - self.v0;
        let c1 = phit - self.v1;
        let c2 = phit - self.v2;

        if (n.dot_product(v0v1.cross_product(c0)) > 0.0 &&
            n.dot_product(v1v2.cross_product(c1)) > 0.0 &&
            n.dot_product(v2v0.cross_product(c2)) > 0.0) {
        
            let intersect = Intersection{
                            hit_p : phit,
                            n : n,};
            Some(intersect)
        }
        else{ return None;}
    }
}
