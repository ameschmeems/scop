extern crate sdl2;
extern crate gl;
// open gl docs locally -> cargo doc -p gl --no-deps --open

pub mod render_gl;
pub mod resources;
pub mod triangle;
pub mod square;
pub mod cube;
pub mod model;
pub mod scene;
pub mod camera;

use resources::Resources;
use std::path::Path;
use sdl2::keyboard::Keycode;

fn main()
{
	let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
	let gl_attr = video_subsystem.gl_attr();
	sdl.mouse().set_relative_mouse_mode(true);

	// let timer = sdl.timer().unwrap();

	// set opengl to core profile
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	// set version to 4.5
	gl_attr.set_context_version(4, 5);

    let window = video_subsystem.window("Scop", 900, 700).opengl().resizable().build().unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

	let mut viewport = render_gl::Viewport::for_window(900, 700);

	// let program = self::render_gl::Program::from_res(&res, "shaders/triangle").unwrap();

	// i fucked up and need to have a seperate copy for each model or it won't compile (fix it pls, future me)
	let program_2 = self::render_gl::Program::from_res(&res, "shaders/triangle").unwrap();

	// let mesh_42 = model::Mesh::from_file("assets/models/42.obj", program);
	let mesh_teapot = model::Mesh::from_file("assets/models/teapot2.obj", program_2);

    unsafe
	{
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

	viewport.set_used();

	unsafe
	{
		// gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
		gl::Enable(gl::DEPTH_TEST);
	}

	// projection matrix for the scene

	let camera = camera::Camera::new();

	let mut scene = scene::Scene::new(vec![mesh_teapot], camera);

    'main: loop
    {
        for event in event_pump.poll_iter()
        {
            // handle user input
            match event
            {
                sdl2::event::Event::Quit { .. } => break 'main,
				sdl2::event::Event::Window {
					win_event: sdl2::event::WindowEvent::Resized(w, h),
					..
				} => {
					viewport.update_size(w, h);
					viewport.set_used();
				},
				sdl2::event::Event::KeyDown {
					keycode: Some(key),
					..
				} => {
					match key
					{
						Keycode::Escape => { break 'main }
						_ => { scene.process_input(&event) }
					}
				},
                _ => { scene.camera.update_camera(&event) },
            }
        }

        unsafe
        {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

		scene.draw();

        window.gl_swap_window();
    }
}
