// absoule import
// from root find std and
// ring into this namespace

use std;
#[derive(Debug,Clone,Copy)]
pub struct Vector{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
// IMPLIMENTATION OF FUNCTIONS
impl Vector {

    // new fn, takes arguments // constuctors of the struct you are 
    // implimenting for.. No Self
    // convention is to call this new... 
    // mainly just creates new version of "struct"
    pub fn new(x:f32,y:f32,z:f32) -> Vector{
        Vector{
            x: x,
            y: y,
            z: z,
        }
    }
    
    // Implimenting Dot Product For Vector
    // Use to find angle between two vectors
    //or used to find the angle between a vector and the 
    // axis of a coordinate system
    pub fn dot_product(self,vec: Vector)-> f32{
        (self.x * vec.x) +            
        (self.y * vec.y) +
        (self.z * vec.z)
    }
    
    // Implimenting Cross Product For Vector
    pub fn cross_product(self,vec: Vector) -> Vector{
        Vector{
            x:((self.y * vec.z) - (self.z * vec.y)),
            y:-((self.x * vec.z) - (self.z * vec.x)),
            z:((self.x * vec.y) -  (self.y * vec.x)),
        }
    }

    // Implimenting Unit Vector for Vector
    pub fn unit(self,vec: Vector) -> Vector{
        let cross = self.cross_product(vec);
        let sqrt = (cross.x.powf(2.0) + 
                    cross.y.powf(2.0) +
                    cross.z.powf(2.0)).abs().sqrt();
        Vector{
             x: (cross.x / sqrt),
             y: (cross.y / sqrt),
             z: (cross.z / sqrt),
        }
    }

    pub fn normalize(self) -> Vector{
        let mag = (self.x.powf(2.0) + 
                   self.y.powf(2.0) +
                   self.z.powf(2.0)).sqrt();
        Vector{
            x: ( self.x / mag),
            y: ( self.y / mag),
            z: ( self.z / mag),
        }

    }
}


// IMPLIMENTATIONS of Extrenal Traits
//  RHS :: SELF ( type that we want to implemnt this op for)

impl std::ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self,rhs:Vector) -> Vector{
        Vector{
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}


impl std::ops::Mul<f32> for Vector{

    type Output =  Vector;
    fn mul(self,rhs: f32 ) -> Vector{
        Vector{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


impl std::ops::Div<f32> for Vector{
    
    type Output = Vector;
    fn div(self,rhs: f32) -> Vector{
        Vector{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


impl std::ops::Add for Vector{

    type Output =  Vector;

    fn add(self,rhs: Vector)->Vector{
        Vector{
        x: self.x + rhs.x,
        y: self.y + rhs.y,
        z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<f32> for Vector{

    type Output =  Vector;

    fn add(self,rhs: f32)->Vector{
        Vector{
        x: self.x + rhs,
        y: self.y + rhs,
        z: self.z + rhs,
        }
    }
}

impl std::ops::Sub for Vector{
    type Output = Vector;
    
    fn sub(self,rhs:Vector) -> Vector{
        Vector{
        x: self.x - rhs.x,
        y: self.y - rhs.y,
        z: self.z - rhs.z,
        }
    }
}


