extern crate sdl2;
extern crate gl;
// open gl docs locally -> cargo doc -p gl --no-deps --open

pub mod render_gl;
pub mod resources;

use resources::Resources;
use std::path::Path;
use render_gl::data;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex
{
	pos: data::f32_f32_f32,
	color: data::f32_f32_f32,
}

impl Vertex
{
	fn vertex_attrib_pointers()
	{
		let stride = std::mem::size_of::<Self>(); // byte offset betweem consecutive attributes

		let location = 0; // layout (location = 0)
		let offset = 0; // offset of the first component

		unsafe
		{
			data::f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
		}

		let location = 1; // layout (location = 1)
		let offset = offset + std::mem::size_of::<data::f32_f32_f32>();

		unsafe
		{
			data::f32_f32_f32::vertex_attrib_pointer(stride, location, offset)
		}
	}
}

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

    unsafe
	{
		gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

	let shader_program = render_gl::Program::from_res(
		&res, "shaders/triangle"
	).unwrap();

	// let vertices: Vec<f32> = vec![
	// 	// positions		// colors
	// 	0.5, -0.5, 0.0,		1.0, 0.0, 0.0,	// bottom right
	// 	-0.5, -0.5, 0.0,	0.0, 1.0, 0.0,	// bottom left
	// 	0.0, 0.5, 0.0,		0.0, 0.0, 1.0	// top
	// ];

	let vertices: Vec<Vertex> = vec![
		Vertex { pos: (0.5, -0.5, 0.0).into(),	color: (1.0, 0.0, 0.0).into() },	// bottom right
		Vertex { pos: (-0.5, -0.5, 0.0).into(),	color: (0.0, 1.0, 0.0).into() },	// bottom left
		Vertex { pos: (0.0, 0.5, 0.0).into(),	color: (0.0, 0.0, 1.0).into() }	// top
	];

	// VBO - vertex buffer object, which lets us upload vertex data to the gpu (position, normal vector, color)
	let mut vbo: gl::types::GLuint = 0;
	// first argument specifies how many VBOs we want, the second is an array that will be overwritten with the VBOs created
	// rust references act as pointers here, so passing &vbo works as long as we only ask for 1 VBO
	// (yes, the 'unsafe' is here for that reason)
	unsafe
	{
		gl::GenBuffers(1, &mut vbo);
	}

	unsafe
	{
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,
			(vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, // size of data in bytes
			vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
			gl::STATIC_DRAW
		);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0); //unbind the buffer
	}

	// create a Vertex Array Object (VAO) which describes how to interpret data in our 'vertices' Vec
	let mut vao: gl::types::GLuint = 0;
	unsafe
	{
		gl::GenVertexArrays(1, &mut vao);
	}
	// Bind the VAO
	unsafe
	{
		gl::BindVertexArray(vao);
		// We bind the VBO as well, to configure the realtion between the two
		// Rebinding it might be wasteful, but its here to make it clear we actually need the VBO now
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

		Vertex::vertex_attrib_pointers();

		// Unbind VAO and VBO
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindVertexArray(0);
	}

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

        // render traingle
		shader_program.set_used();
		unsafe
		{
			gl::BindVertexArray(vao);
			gl::DrawArrays(
				gl::TRIANGLES,
				0,
				3
			);
		}

        window.gl_swap_window();
    }
}

