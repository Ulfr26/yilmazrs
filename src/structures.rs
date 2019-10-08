extern crate nalgebra as na;

use na::{Vector4,Matrix4};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Position and direction vectors: [x, y, z, w]
// w indicates position vs direction: w=1 for position, w=0 for direction.

#[derive(Debug, Clone)]
pub struct Face {
    pub vertices: [usize; 3],
    pub normal: Vector4<f64>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vector4<f64>>,
    pub faces: Vec<Face>,
    pub pos: Vector4<f64>,
    pub rot: Vector4<f64>,
    pub scale: f64,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            name: String::new(),
            vertices: Vec::new(),
            faces: Vec::new(),
            pos: Vector4::w(),
            rot: Vector4::zeros(),
            scale: 1.0,
        }
    }

    pub fn from_file(filename: String) -> Result<Mesh, String> {
        // Creates a mesh from a wavefront OBJ file.
        // It must have faces defined as triangles, and will ignore most stuff for now.
        
        let f = File::open(filename);

        if let Ok(file) = f {
            let mut res = Mesh::new();
            let reader = BufReader::new(file);
            let mut normals: Vec<Vector4<f64>> = Vec::new();

            for (_, l) in reader.lines().enumerate() {
                let lstr = l.unwrap();
                let line: Vec<&str> = lstr.split(" ").collect();
                
                match line[0] {
                    "o" => {
                        // The name of the mesh
                        res.name = line[1].to_string();
                    },

                    "v" => {
                        let v: (f64, f64, f64) = (
                            line[1].parse().unwrap(),
                            line[2].parse().unwrap(),
                            line[3].parse().unwrap(),
                        );
                        res.vertices.push(Vector4::new(v.0, v.1, v.2, 1.0));
                    },

                    "vn" => {
                        let vn: (f64, f64, f64) = (
                            line[1].parse().unwrap(),
                            line[2].parse().unwrap(),
                            line[3].parse().unwrap(),
                        );

                        normals.push(Vector4::new(vn.0, vn.1, vn.2, 0.0));
                    },

                    "f" => {
                        let mut f = [0usize; 3];
                        let mut norm = 0usize;

                        for i in 1..line.len() {
                            // Getting really fricking tired of having to make  a heap of 
                            // new variables just to avoid borrowing issues, rust
                            let _l = line[i].to_string();
                            let l: Vec<&str> = _l.split("//").collect();
                            f[i-1] = l[0].parse::<usize>().unwrap() - 1;

                            norm = l[1].parse::<usize>().unwrap() - 1;
                        }

                        res.faces.push(
                            Face {
                                vertices: f,
                                normal: normals[norm],
                            }
                        );
                    },

                    &_ => {}
                }
            }

            return Ok(res);
        } 
        
        else {
            return Err(String::from("Error while opening mesh file."));
        }
    }
}
