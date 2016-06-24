#[derive(Debug,Clone,Copy)]
struct Vector{
    x: f32,
    y: f32,
    z: f32,

}

// IMPLIMENTATION OF FUNCTIONS
impl Vector {

    // new fn, takes arguments // constuctors of the struct you are 
    // implimenting for.. No Self
    // convention is to call this new... 
    // mainly just creates new version of "struct"
    fn new(x:f32,y:f32,z:f32) -> Vector{
        Vector{
            x: x,
            y: y,
            z: z,
        }
    }
    
    // Implimenting Dot Product For Vector
    fn dot(self,vec: Vector)-> f32{
        (self.x * vec.x) +            
        (self.y * vec.y) +
        (self.z * vec.z)
    }
    
    // Implimenting Cross Product For Vector
    fn cross(self,vec: Vector) -> Vector{
        Vector{
            x:((self.y * vec.z) - (self.z * vec.y)),
            y:-((self.x * vec.z) - (self.z * vec.x)),
            z:((self.x * vec.y) -  (self.y * vec.x)),
        }
    }

    // Implimenting Unit Vector for Vector
    fn unit(self,vec: Vector) -> Vector{
        let cross = self.cross(vec);
        let sqrt = (cross.x.powf(2.0) + 
                    cross.y.powf(2.0) +
                    cross.z.powf(2.0)).abs().sqrt();
        Vector{
             x: (cross.x / sqrt),
             y: (cross.y / sqrt),
             z: (cross.z / sqrt),
        }
    }


}


// IMPLIMENTATIONS of Extrenal Traits

//                 RHS        SELF ( type that we want to implemnt this op for)

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



fn main() {
    let vec1 = Vector{x: 1.2,y: 15.2,z: 225.3};
    let vec2 = Vector::new(3.5,235.3,352.0);
    let float = 2.24;

// print statements
    println!("Vecotr: {:?} + Vector : {:?} = {:?}", vec1,vec2,(vec1+vec2));
    
    println!("");
    println!("Vecotr: {:?} - Vector : {:?} = {:?}", vec1,vec2,(vec1-vec2));

    println!("");
    println!("Vecotr: {:?} * Float : {:?} = {:?}", vec1,float ,(vec1*float));

    println!("");
    println!("Float: {:?} * Vector : {:?} = {:?}", float, vec1, (float * vec1));
    
    println!(",");
    println!(" {:?} o {:?} = {:?}", vec1, vec2, vec1.dot(vec2));
    // println!(" {:?} o {:?} = {:?}", vec1, vec2, Vector::dot(vec1, vec2));

    println!(",");
    println!(" {:?} x {:?} = {:?}", vec1, vec2, vec1.cross(vec2));

    println!(",");
    println!("Unit Vec of {:?} and {:?} = {:?}", vec1, vec2, vec1.unit(vec2));

}
