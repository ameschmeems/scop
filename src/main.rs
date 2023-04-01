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

	let vertices: Vec<f32> = vec![
		// positions		// colors
		0.5, -0.5, 0.0,		1.0, 0.0, 0.0,	// bottom right
		-0.5, -0.5, 0.0,	0.0, 1.0, 0.0,	// bottom left
		0.0, 0.5, 0.0,		0.0, 0.0, 1.0	// top
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
			(vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
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
		// Specify the data layout for attribute 0
		gl::EnableVertexAttribArray(0);
		gl::VertexAttribPointer(
			0, // index of the generic vertex attribute
			3, // Number of components per generic vertex attribute
			gl::FLOAT,
			gl::FALSE,
			(6 * std::mem::size_of::<f32>()) as gl::types::GLint, // offset between consecutive attributes
			std::ptr::null() // offset of the first component
		);
		// Specify the data layout for attribute 1
		gl::EnableVertexAttribArray(1);
		gl::VertexAttribPointer(
			1, // index of the generic vertex attribute
			3, // Number of components per generic vertex attribute
			gl::FLOAT,
			gl::FALSE,
			(6 * std::mem::size_of::<f32>()) as gl::types::GLint, // offset between consecutive attributes
			(3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
		);

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

