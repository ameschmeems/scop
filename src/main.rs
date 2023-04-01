extern crate sdl2;
extern crate gl;
// open gl docs locally -> cargo doc -p gl --no-deps --open

pub mod render_gl;

fn main()
{
    // println!("Hello, world!");
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

    unsafe
	{
		gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

	// leave the use here, bc ideally the following code will be moved to another file
	use std::ffi::CString;

	let vert_shader = render_gl::Shader::from_vert_source(
		&CString::new(include_str!("triangle.vert")).unwrap()
	).unwrap();

	let frag_shader = render_gl::Shader::from_frag_source(
		&CString::new(include_str!("triangle.frag")).unwrap()
	).unwrap();

	let shader_program = render_gl::Program::from_shaders(
		&[vert_shader, frag_shader]
	).unwrap();

	shader_program.set_used();

    'main: loop
    {
        for event in event_pump.poll_iter()
        {
            //handle user input
            match event
            {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {},
            }
        }

        unsafe
        {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();
        // render contents
    }
}

