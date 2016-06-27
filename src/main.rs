//extern crate rust_training;
//use rust_training::vectors::maths;

mod vectors;
use vectors::maths::Vector;

fn main() {
    let vec1 = Vector::new(1.2,15.2,225.3);
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
