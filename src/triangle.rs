use crate::resources::Resources;
use crate::render_gl::{self, buffer, data, texture::Texture};

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex
{
	pub pos: data::f32_f32_f32,
	pub color: data::f32_f32_f32,
	pub texture: data::f32_f32,
}

impl Vertex
{
	pub fn vertex_attrib_pointers()
	{
		let stride = std::mem::size_of::<Self>(); // byte offset between consecutive attributes

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

		// println!("stride: {}, offset: {}", stride, offset);
		let location = 2; // layout (location = 2)
		let offset = offset + std::mem::size_of::<data::f32_f32_f32>();

		unsafe
		{
			data::f32_f32::vertex_attrib_pointer(stride, location, offset);
		}
	}
}


pub struct Triangle
{
	program: render_gl::Program,
	_vbo: buffer::ArrayBuffer,
	vao: buffer::VertexArray,
	tex: Texture,
}

impl Triangle
{
	pub fn new(res: &Resources, tex_path: &str) -> Result<Self, render_gl::Error>
	{
		let program = render_gl::Program::from_res(res, "shaders/triangle")?;


		let vertices: Vec<Vertex> = vec![
			Vertex { pos: (0.5, -0.5, 0.0).into(),	color: (1.0, 0.0, 0.0).into(), texture: (1.0, 0.0).into() },	// bottom right
			Vertex { pos: (-0.5, -0.5, 0.0).into(),	color: (0.0, 1.0, 0.0).into(), texture: (0.0, 0.0).into() },	// bottom left
			Vertex { pos: (0.0, 0.5, 0.0).into(),	color: (0.0, 0.0, 1.0).into(), texture: (0.5, 1.0).into() }	// top
		];

		// VBO - vertex buffer object, which lets us upload vertex data to the gpu (position, normal vector, color)
		// Need to use const generic parameters in {} !!!! IMPORTANT !!!!
		let vbo = buffer::ArrayBuffer::new();
		vbo.bind();
		vbo.static_draw_data(&vertices);
		vbo.unbind();

		// create a Vertex Array Object (VAO) which describes how to interpret data in our 'vertices' Vec
		let vao = buffer::VertexArray::new();

		let tex = Texture::new();
		// tex.set_wrapping(gl::REPEAT);
		// tex.set_filtering(gl::NEAREST);
		tex.load(tex_path);

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
			vao,
			tex
		})
	}

	pub fn render(&self)
	{
		self.tex.activate(gl::TEXTURE0);
		self.program.set_used();
		self.tex.bind();
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