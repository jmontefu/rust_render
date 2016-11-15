use primitives::triangle::Triangle;
use math::vectors::Vector;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;


#[derive(Debug)]
pub struct Mesh {

    pub triangles: Vec<Triangle>
    
}


impl Mesh{

    pub fn new() -> Mesh{
        Mesh{
            triangles: Vec::new(),
        }
    }

    
    // Result
    // to make it safer incase we dont pass
    // a correct value in "path" or the file is 
    // invaled // permisions. 

    // use match 
    // same as option // some



    pub fn read( mut self,path: &str ) -> Mesh{
        let file =  File::open(
                    Path::new(path)).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines(){
            
            let unwrap = line.unwrap();
            let vert_p = unwrap.split_whitespace().collect::<Vec<_>>();

            let v0 = Vector::new(
                        vert_p[0].parse::<f32>().unwrap(),
                        vert_p[1].parse::<f32>().unwrap(),
                        vert_p[2].parse::<f32>().unwrap());

            let v1 = Vector::new(
                        vert_p[3].parse::<f32>().unwrap(),
                        vert_p[4].parse::<f32>().unwrap(),
                        vert_p[5].parse::<f32>().unwrap());

            let v2 = Vector::new(
                        vert_p[6].parse::<f32>().unwrap(),
                        vert_p[7].parse::<f32>().unwrap(),
                        vert_p[8].parse::<f32>().unwrap());

            let tri = Triangle{
                               v0: v0, 
                               v1: v1,
                               v2: v2,
                               };
            

            println!{"{:?}",tri};
            self.triangles.push(tri);
            }
        
        return self;
    }

}

