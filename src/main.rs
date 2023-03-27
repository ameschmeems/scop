extern crate sdl2;

fn main() {
    // println!("Hello, world!");
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem.window("Scop", 900, 700).resizable().build().unwrap();
    loop {
        
    }
}
