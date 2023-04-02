use crate::resources::Resources;
use crate::render_gl::{self, buffer, data};

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


pub struct Triangle
{
	program: render_gl::Program,
	_vbo: buffer::ArrayBuffer,
	vao: buffer::VertexArray,
}

impl Triangle
{
	pub fn new(res: &Resources) -> Result<Self, render_gl::Error>
	{
		let program = render_gl::Program::from_res(res, "shaders/triangle")?;

		let vertices: Vec<Vertex> = vec![
			Vertex { pos: (0.5, -0.5, 0.0).into(),	color: (1.0, 0.0, 0.0).into() },	// bottom right
			Vertex { pos: (-0.5, -0.5, 0.0).into(),	color: (0.0, 1.0, 0.0).into() },	// bottom left
			Vertex { pos: (0.0, 0.5, 0.0).into(),	color: (0.0, 0.0, 1.0).into() }	// top
		];

		// VBO - vertex buffer object, which lets us upload vertex data to the gpu (position, normal vector, color)
		// Need to use const generic parameters in {} !!!! IMPORTANT !!!!
		let vbo = buffer::ArrayBuffer::new();
		vbo.bind();
		vbo.static_draw_data(&vertices);
		vbo.unbind();

		// create a Vertex Array Object (VAO) which describes how to interpret data in our 'vertices' Vec
		let vao = buffer::VertexArray::new();

		// Bind the VAO
		vao.bind();
		// We bind the VBO as well, to configure the realtion between the two
		// Rebinding it might be wasteful, but its here to make it clear we actually need the VBO now
		vbo.bind();

		Vertex::vertex_attrib_pointers();

		// Unbind VAO and VBO
		vbo.unbind();
		vao.unbind();

		Ok(Triangle {
			program,
			_vbo: vbo,
			vao
		})
	}

	pub fn render(&self)
	{
		self.program.set_used();
		self.vao.bind();

		unsafe
		{
			gl::DrawArrays(
				gl::TRIANGLES,
				0,
				3
			);
		}
	}
}