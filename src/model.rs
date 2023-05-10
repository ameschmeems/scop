use math;
use gl;
use crate::render_gl::{self, buffer, texture};

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex
{
	position: math::vector::Vector3,
	normal: math::vector::Vector3,
	texcoord: math::vector::Vector2,
}

impl Vertex
{
	pub fn new(position: math::vector::Vector3) -> Self
	{
		Self {
			position,
			normal: (0.0, 0.0, 0.0).into(),
			texcoord: (0.0, 0.0).into()
		}
	}
}

pub struct Mesh
{
	vertices: Vec<Vertex>,
	indices: Vec<u32>,
	// textures: Vec<texture::Texture>,
	vao: buffer::VertexArray,
	vbo: buffer::ArrayBuffer,
	ebo: buffer::ElementArrayBuffer,
	program: render_gl::Program
}

impl Mesh
{
	// basic constructor from raw data (could use it to re-create the triangla/square/cube objects)
	pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, program: render_gl::Program) -> Self
	{
		let vao = buffer::VertexArray::new();
		let ebo = buffer::ElementArrayBuffer::new();
		let vbo = buffer::ArrayBuffer::new();

		let mesh = Mesh {
			vertices,
			indices,
			// textures,
			vao,
			vbo,
			ebo,
			program
		};

		mesh.setup_mesh();

		mesh
	}

	fn setup_mesh(&self)
	{
		self.vao.bind();
		self.vbo.bind();

		self.vbo.static_draw_data(self.vertices.as_slice());

		self.ebo.bind();
		self.ebo.static_draw_data(self.indices.as_slice());

		unsafe
		{
			// vertex positions
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				0,
				3,
				gl::FLOAT,
				gl::FALSE,
				std::mem::size_of::<Vertex>() as gl::types::GLint,
				0 as *const gl::types::GLvoid
			);

			// vertex normals
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				1,
				3,
				gl::FLOAT,
				gl::FALSE,
				std::mem::size_of::<Vertex>() as gl::types::GLint,
				std::mem::size_of::<math::vector::Vector3>() as *const gl::types::GLvoid
			);

			// vertex texture coords
			gl::EnableVertexAttribArray(2);
			gl::VertexAttribPointer(
				2,
				2,
				gl::FLOAT,
				gl::FALSE,
				std::mem::size_of::<Vertex>() as gl::types::GLint,
				(std::mem::size_of::<math::vector::Vector3>() * 2) as *const gl::types::GLvoid
			);
		}
		self.vao.unbind();
	}

	pub fn render(&self)
	{
		self.program.set_used();
		self.vao.bind();

		unsafe
		{
			gl::DrawElements(
				gl::TRIANGLES,
				self.indices.len() as gl::types::GLint,
				gl::UNSIGNED_INT,
				0 as *const gl::types::GLvoid
			);
		}

		self.vao.unbind();
	}
}