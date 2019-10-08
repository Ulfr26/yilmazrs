// Ah fricc, here we go again
// I have attempted this many times
// this time it will work!
// (because I am using rust)
// (and not C)
//
// So this is essentially a 3d rendering engine in the terminal. It is not fast, it is not
// efficient, it is not good. But hopefully, it will work.

mod screen;
mod structures;
extern crate nalgebra as na;

use screen::{Screen, Colour, Camera};
use structures::Mesh;
use na::core::{Matrix4, Vector4, Vector3};
use std::thread;
use std::time::Duration;

fn main() {
    let mut object = Mesh::from_file("./objects/monkey.obj".to_string()).unwrap();
    object.pos = Vector4::new(-100.0, 200.0, -2.0, 1.0);
    object.rot = Vector4::new(0.0, 0.0, 0.0, 0.0);
    object.scale = 200.0;

    let mut camera = Camera::new();
    // Ha ha ha
    // ...
    // i don't know how to slice vectors
    camera.target = Vector3::new(object.pos[0], object.pos[1], object.pos[2]);

    let mut screen = Screen::new(camera);
    screen.meshes = vec![object];

    let mut rot = 0.0;

    loop {
        screen.update_size();
        screen.meshes[0].scale = (screen.height as f64/2.0);
        screen.background(Colour::Rgb(0, 0, 0));
        screen.render();
        screen.print_screen();

        rot += 0.5;
        screen.meshes[0].rot[0] = rot;
        screen.meshes[0].rot[1] = rot;

        thread::sleep(Duration::from_millis(200));
    }

    print!("\x1B[2J");
}
