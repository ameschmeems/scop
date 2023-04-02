extern crate sdl2;
extern crate gl;
// open gl docs locally -> cargo doc -p gl --no-deps --open

pub mod render_gl;
pub mod resources;
pub mod triangle;

use resources::Resources;
use std::path::Path;

fn main()
{
	let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
	let gl_attr = video_subsystem.gl_attr();

	// set opengl to core profile
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	// set version to 4.5
	gl_attr.set_context_version(4, 5);

    let window = video_subsystem.window("Scop", 900, 700).opengl().resizable().build().unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

	let mut viewport = render_gl::Viewport::for_window(900, 700);

    unsafe
	{
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

	let triangle = triangle::Triangle::new(&res).unwrap();

	viewport.set_used();

    'main: loop
    {
        for event in event_pump.poll_iter()
        {
            //handle user input
            match event
            {
                sdl2::event::Event::Quit { .. } => break 'main,
				sdl2::event::Event::Window {
					win_event: sdl2::event::WindowEvent::Resized(w, h),
					..
				} => {
					viewport.update_size(w, h);
					viewport.set_used();
				}
                _ => {},
            }
        }

        unsafe
        {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

		triangle.render();

        window.gl_swap_window();
    }
}

