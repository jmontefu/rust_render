
#[derive(Debug,Clone,Copy)]
struct Vector{
    x: f32,
    y: f32,
    z: f32,

}



impl std::ops::Mul<f32> for Vector{

    type Output =  Vector;
    fn mul(self,rhs: f32 )->Vector{
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



fn main() {
    let vec1 = Vector{x: 1.2,y: 15.2,z: 225.3};
    let vec2 = Vector{x: 2.2,y: 32.3,z: 33.33};
    let float = 2.24;

    println!("Vecotr: {:?} + Vector : {:?} = {:?}", vec1,vec2,(vec1+vec2));

    println!("Vecotr: {:?} * Float : {:?} = {:?}", vec1,float ,(vec1*float));

}
