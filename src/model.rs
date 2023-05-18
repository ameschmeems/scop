use math;
use gl;
use math::vector::Vector3;
use std::ffi::CString;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::fs::File;

use crate::render_gl::{self, buffer};

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


	pub fn from_file<T>(filename: T, program: render_gl::Program) -> Self
	where T: AsRef<Path>
	{
		let vao = buffer::VertexArray::new();
		let ebo = buffer::ElementArrayBuffer::new();
		let vbo = buffer::ArrayBuffer::new();

		let mut temp_vertices = Vec::<math::vector::Vector3>::new();
		let mut temp_texcoords = Vec::<math::vector::Vector2>::new();
		let mut temp_normals = Vec::<math::vector::Vector3>::new();

		let mut indices = Vec::<u32>::new();

		if let Ok(lines) = read_lines(filename)
		{
			for line in lines
			{
				if let Ok(vertex) = line
				{
					let arr: Vec<&str> = vertex.split(" ").collect();
					// println!("{:?}", elements);
					if arr[0] == "v"
					{
						let tmp = math::vector::Vector3::new(arr[1].parse().unwrap(), arr[2].parse().unwrap(), arr[3].parse().unwrap());
						temp_vertices.push(tmp);
					}
					else if arr[0] == "vt"
					{
						let tmp = math::vector::Vector2::new(arr[1].parse().unwrap(), arr[2].parse().unwrap());
						temp_texcoords.push(tmp);
					}
					else if arr[0] == "vn"
					{
						let tmp = math::vector::Vector3::new(arr[1].parse().unwrap(), arr[2].parse().unwrap(), arr[3].parse().unwrap());
						temp_normals.push(tmp);
					}
					else if arr[0] == "f"
					{
						let tmp1: u32 = arr[1].parse().unwrap();
						let tmp2: u32 = arr[2].parse().unwrap();
						let tmp3: u32 = arr[3].parse().unwrap();
						indices.push(tmp1 - 1);
						indices.push(tmp2 - 1);
						indices.push(tmp3 - 1);
					}
				}
			}
		}
		// println!("{:?}", temp_vertices);
		// println!("{:?}", temp_indices);
		let mut vertices = Vec::<Vertex>::new();
		for i in temp_vertices
		{
			vertices.push(Vertex::new(i));
		}

		// println!("{:?}", vertices);
		// println!("{:?}", indices);

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

	pub fn render(&self, camera_pos: &Vector3, camera_front: &Vector3, camera_up: &Vector3)
	{
		let model = math::matrix::Matrix4::new_identity();
		let model = math::rotate(&model, 45.0f32.to_radians(), &(0.5, 1.0, 0.0).into());

		let view = math::look_at(
			camera_pos,
			&(*camera_pos + *camera_front),
			camera_up
		);

		let projection = math::matrix::Matrix4::new_perspective(45.0f32.to_radians(), 900.0/700.0, 0.1, 100.0);

		let model_location = unsafe {
			let string = CString::new("model").unwrap();
			gl::GetUniformLocation(self.program.id(), string.as_ptr())
		};

		let view_location = unsafe {
			let string = CString::new("view").unwrap();
			gl::GetUniformLocation(self.program.id(), string.as_ptr())
		};

		let projection_location = unsafe {
			let string = CString::new("projection").unwrap();
			gl::GetUniformLocation(self.program.id(), string.as_ptr())
		};

		self.program.set_used();

		unsafe
		{
			// Need to transpose the matrix before passing to the shader, as opengl expects numbers in columns, and we save numbers in rows
			gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &model.transposed() as *const math::matrix::Matrix4 as *const f32);
			gl::UniformMatrix4fv(view_location, 1, gl::FALSE, &view.transposed() as *const math::matrix::Matrix4 as *const f32);
			gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, &projection.transposed() as *const math::matrix::Matrix4 as *const f32);
		}

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

// Returns iterator over lines of a file
fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where T: AsRef<Path>,
{
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}