use terminal_size::{Width, Height, terminal_size};
use std::cmp::{max, min};
use crate::structures::Mesh; use nalgebra::core::{Vector3, Vector4, Matrix4};
use nalgebra::geometry::Point3;

#[derive(Clone, Copy, Debug)]
pub enum Colour {
    Grey(u8),
    Rgb(u8,u8,u8)
}

pub struct Camera {
    pub pos: Vector3<f64>,
    pub target: Vector3<f64>,
    pub up: Vector3<f64>,
}

pub struct Screen {
    pub screen: Vec<Colour>,
    pub width: usize,
    pub height: usize,
    pub meshes: Vec<Mesh>,
    pub camera: Camera,
}

// Fricc it, I'm defining some of my own matrix functions.
fn translation_from(pos: &Vector4<f64>) -> Matrix4<f64> {
    return Matrix4::new(
        1.0, 0.0, 0.0, pos[0],
        0.0, 1.0, 0.0, pos[1],
        0.0, 0.0, 1.0, pos[2],
        0.0, 0.0, 0.0, 1.0
    );
}

fn scaling_from(s: f64) -> Matrix4<f64> {  
    return Matrix4::new(
        s, 0.0, 0.0, 0.0,
        0.0, s, 0.0, 0.0,
        0.0, 0.0, s, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            pos: Vector3::zeros(),
            target: Vector3::zeros(),
            up: Vector3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn from(pos: Vector3<f64>, target: Vector3<f64>, up: Vector3<f64>) -> Camera {
        Camera {
            pos: pos,
            target: target,
            up: up,
        }
    }
}

impl Screen {
    pub fn new(camera: Camera) -> Screen {
        let width: usize;
        let height: usize;

        let size = terminal_size();

        if let Some((Width(w), Height(h))) = size {
            width = w as usize;
            height = h as usize;
        }

        else {
            println!("Heckers, this shouldn't happen");    
            return Screen::from(0,0, Vec::new(), camera);
        }

        return Screen::from(width, height, Vec::new(), camera);
    }

    pub fn new_with_meshes(meshes: Vec<String>, camera: Camera) -> Screen {
        let mut res = Screen::new(camera);
        
        for filename in meshes {
            res.meshes.push(Mesh::from_file(filename).unwrap());
        }

        return res;
    }

    pub fn update_size(&mut self) {
        let size = terminal_size();
        
        if let Some((Width(w), Height(h))) = size {
            self.width = w as usize;
            self.height = h as usize;
        }

        else {
            println!("oh no");
        }
    }

    pub fn from(width: usize, height: usize, meshes: Vec<String>, camera: Camera) -> Screen {
        let mut res = Screen {
            screen: Vec::new(),
            width: width,
            height: height,
            meshes: Vec::new(),
            camera: camera,
        };

        for i in 0..width*height {
            res.screen.push(Colour::Rgb(0,0,0));
        }

        for filename in meshes {
            res.meshes.push(Mesh::from_file(filename).unwrap());
        }

        return res;
    }

    pub fn print_screen(&self) {
        print!("\x1B[1;1h");
        print!("\x1B[2J");

        for i in 0..self.screen.len() {
            match self.screen[i] {
                Colour::Rgb(r,g,b) => {
                    print!("\x1B[38;2;{};{};{}m", r, g, b);
                    print!("█");
                },

                Colour::Grey(l) => {
                    print!("\x1B[38;2;{};{};{}m", l, l, l);
                    print!("█");
                }

                _ => {}
            }
        }
    }

    pub fn bresenham(&mut self, p1: (f64,f64), p2: (f64,f64), c: Colour) {
        // Straight up "adapted" from wikipedia
        // (As is most of this project to be fair)
        if (p2.1-p1.1).abs() < (p2.0-p1.0).abs() {
            if p1.0 > p2.0 {
                self.bresenham_low(p2, p1, c);
            } else {
                self.bresenham_low(p1, p2, c);
            }
        } else {
            if p1.1 > p2.1 {
                self.bresenham_high(p2, p1, c);
            } else {
                self.bresenham_high(p1, p2, c);
            }
        }
    }

    fn bresenham_low(&mut self, p1: (f64,f64), p2: (f64,f64), c: Colour) {
        let dx = p2.0 - p1.0;
        let mut dy = p2.1 - p1.1;

        let mut yi = 1.0;

        if dy < 0.0 {
            yi = -1.0;
            dy *= -1.0;
        }

        let mut d = 2.0*dy - dx;
        let mut y = p1.1;

        let x0 = 0.0_f64.max(p1.0).min(self.width as f64-1.0);
        let x1 = 0.0_f64.max(p2.0+1.0).min(self.width as f64-1.0);

        for x in (x0 as usize)..(x1 as usize) {
            self.set_pixel(x, y as usize, c); 

            if d > 0.0 {
                y += yi;
                d -= 2.0*dx;
            }

            d += 2.0*dy;
        }
    }

    fn bresenham_high(&mut self, p1: (f64,f64), p2: (f64,f64), c: Colour) {
        let mut dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;

        let mut xi = 1.0;

        if dx < 0.0 {
            xi = -1.0;
            dx *= -1.0;
        }

        let mut d = 2.0*dx - dy;
        let mut x = p1.0;

        let y0 = 0.0_f64.max(p1.1).min(self.height as f64-1.0);
        let y1 = 0.0_f64.max(p2.1+1.0).min(self.height as f64-1.0);

        for y in (y0 as usize)..(y1 as usize) {
            self.set_pixel(x as usize, y as usize, c); 

            if d > 0.0 {
                x += xi;
                d -= 2.0*dy;
            }

            d += 2.0*dx;
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: Colour) {
        // The board goes in a fairly normal left-to-right downwards fashion because that's how
        // it's printed. Thus we have self.height rows of self.width columns, going from 0 to
        // self.width-1.
        
        // Deal with x, y out of bounds
        let x1 = max(0, min(x, self.width-1)); 
        let y1 = max(0, min(y, self.height-1));
        
        let index = self.width*y1 + x1;

        self.screen[index] = c;
    }

    pub fn background(&mut self, c: Colour) {
        for i in 0..self.width*self.height {
            self.screen[i] = c;
        }
    }

    pub fn triangle(&mut self, p1: (f64,f64), p2: (f64,f64), p3: (f64,f64), c: Colour, antialiased: bool) {
        if antialiased {
            // TODO
        } else {
            self.bresenham(p1, p2, c);
            self.bresenham(p2, p3, c);
            self.bresenham(p3, p1, c);
        }

        // TODO: Fill the triangle.
    }

    pub fn render(&mut self) {
        // Model, view and projection matrices:
        // luckily, nalgebra can do it for us
        
        for i in 0..self.meshes.len() {
            let mesh = self.meshes[i].clone();
            let model: Matrix4<f64> = translation_from(&mesh.pos) * Matrix4::from_euler_angles(mesh.rot[0], mesh.rot[1], mesh.rot[2]) * scaling_from(mesh.scale);
            let view: Matrix4<f64> = Matrix4::new_observer_frame(&Point3::from(self.camera.pos), &Point3::from(self.camera.target), &self.camera.up);
            let projection: Matrix4<f64> = Matrix4::new_perspective(self.width as f64/self.height as f64, 3.14/4.0, 1.0, 100.0);

            let mvp: Matrix4<f64> = projection * view * model; 

            for face in &mesh.faces {
                let mut f = [(0.0, 0.0); 3];

                for i in 0..face.vertices.len() {
                    let v = mvp * mesh.vertices[face.vertices[i]];

                    f[i] = (v[0], v[1]/6.0);
                }

                self.triangle(f[0], f[1], f[2], Colour::Rgb(0xFF, 0xFF, 0xFF), false);
            }
        }
    }
}
