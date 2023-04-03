use crate::resources::Resources;
use crate::render_gl::{self, buffer, data};
use crate::triangle::Vertex;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Index
{
	indices: data::uint_uint_uint
}

pub struct Square
{
	program: render_gl::Program,
	_ebo: buffer::ElementArrayBuffer,
	_vbo: buffer::ArrayBuffer,
	vao: buffer::VertexArray
}

impl Square
{
	pub fn new(res: &Resources) -> Result<Self, render_gl::Error>
	{
		let program = render_gl::Program::from_res(res, "shaders/triangle")?;

		let vertices: Vec<Vertex> = vec![
			Vertex { pos: (0.5, 0.5, 0.0).into(), color: (0.0, 0.0, 1.0).into() },
			Vertex { pos: (0.5, -0.5, 0.0).into(), color: (0.0, 1.0, 0.0).into() },
			Vertex { pos: (-0.5, -0.5, 0.0).into(), color: (0.0, 0.0, 1.0).into() },
			Vertex { pos: (-0.5, 0.5, 0.0).into(), color: (1.0, 0.0, 0.0).into() }
		];

		let indices: Vec<Index> = vec![
			Index { indices: (0, 1, 3).into() },
			Index { indices: (1, 2, 3).into() }
		];

		let vao = buffer::VertexArray::new();

		let ebo = buffer::ElementArrayBuffer::new();

		let vbo = buffer::ArrayBuffer::new();

		vao.bind();

		vbo.bind();
		vbo.static_draw_data(&vertices);

		ebo.bind();
		ebo.static_draw_data(&indices);

		Vertex::vertex_attrib_pointers();

		vbo.unbind();
		vao.unbind();
		ebo.unbind();

		Ok(Square {
			program,
			_ebo: ebo,
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
			gl::DrawElements(
				gl::TRIANGLES,
				6,
				gl::UNSIGNED_INT,
				0 as *const gl::types::GLvoid
			)
		}
	}
}