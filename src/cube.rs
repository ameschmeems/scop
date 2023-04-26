use crate::resources::Resources;
use crate::render_gl::{self, buffer, data, texture::Texture};
use crate::triangle::Vertex;
use std::ffi::CString;
extern crate math;
use math::vector::Vector3;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Index
{
	indices: data::uint_uint_uint
}

pub struct Cube
{
	program: render_gl::Program,
	_ebo: buffer::ElementArrayBuffer,
	_vbo: buffer::ArrayBuffer,
	vao: buffer::VertexArray,
	tex: Texture
}

impl Cube
{
	pub fn new(res: &Resources, tex_path: &str) -> Result<Self, render_gl::Error>
	{
		let program = render_gl::Program::from_res(res, "shaders/triangle")?;

		let vertices: Vec<Vertex> = vec![
			Vertex { pos: (-0.5, -0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 0.0).into() },
			Vertex { pos: (0.5, -0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 0.0).into() },
			Vertex { pos: (0.5, 0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 1.0).into() },
			Vertex { pos: (0.5, 0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 1.0).into() },
			Vertex { pos: (-0.5, 0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 1.0).into() },
			Vertex { pos: (-0.5, -0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 0.0).into() },

			Vertex { pos: (-0.5, -0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 0.0).into() },
			Vertex { pos: (0.5, -0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 0.0).into() },
			Vertex { pos: (0.5, 0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 1.0).into() },
			Vertex { pos: (0.5, 0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 1.0).into() },
			Vertex { pos: (-0.5, 0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 1.0).into() },
			Vertex { pos: (-0.5,-0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 0.0).into() },

			Vertex { pos: (-0.5, 0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 0.0).into() },
			Vertex { pos: (-0.5, 0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 1.0).into() },
			Vertex { pos: (-0.5, -0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 1.0).into() },
			Vertex { pos: (-0.5, -0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 1.0).into() },
			Vertex { pos: (-0.5, -0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 0.0).into() },
			Vertex { pos: (-0.5, 0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 0.0).into() },

			Vertex { pos: (0.5, 0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 0.0).into() },
			Vertex { pos: (0.5, 0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 1.0).into() },
			Vertex { pos: (0.5, -0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 1.0).into() },
			Vertex { pos: (0.5, -0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 1.0).into() },
			Vertex { pos: (0.5, -0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 0.0).into() },
			Vertex { pos: (0.5, 0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 0.0).into() },

			Vertex { pos: (-0.5, -0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 1.0).into() },
			Vertex { pos: (0.5, -0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 1.0).into() },
			Vertex { pos: (0.5, -0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 0.0).into() },
			Vertex { pos: (0.5, -0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 0.0).into() },
			Vertex { pos: (-0.5,-0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 0.0).into() },
			Vertex { pos: (-0.5, -0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 1.0).into() },

			Vertex { pos: (-0.5, 0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 1.0).into() },
			Vertex { pos: (0.5, 0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 1.0).into() },
			Vertex { pos: (0.5, 0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 0.0).into() },
			Vertex { pos: (0.5, 0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (1.0, 0.0).into() },
			Vertex { pos: (-0.5, 0.5, 0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 0.0).into() },
			Vertex { pos: (-0.5, 0.5, -0.5).into(), color: (0.0, 0.0, 0.0).into(), texture: (0.0, 1.0).into() },
		];

		// let indices: Vec<Index> = vec![
		// 	Index { indices: (0, 1, 3).into() },
		// 	Index { indices: (1, 2, 3).into() }
		// ];

		let vao = buffer::VertexArray::new();

		let ebo = buffer::ElementArrayBuffer::new();

		let vbo = buffer::ArrayBuffer::new();

		let tex = Texture::new();

		tex.load(tex_path);

		vao.bind();

		vbo.bind();
		vbo.static_draw_data(&vertices);

		// ebo.bind();
		// ebo.static_draw_data(&indices);

		Vertex::vertex_attrib_pointers();

		vbo.unbind();
		vao.unbind();
		// ebo.unbind();

		Ok(Cube {
			program,
			_ebo: ebo,
			_vbo: vbo,
			vao,
			tex
		})
	}

	pub fn render(&self, _time: f32, camera_pos: &Vector3, camera_front: &Vector3, camera_up: &Vector3)
	{
		// different transformations are here\
		// let time = time * 0.001;

		let model = math::matrix::Matrix4::new_identity();
		// let model = math::rotate(&model, -55.0f32.to_radians(), &((1.0, 0.0, 0.0).into()));
		// let model = math::rotate(&model, * 0.1f32.to_radians(), &(0.5, 1.0, 0.0).into());

		// let radius: f32 = 10.0;
		// let cam_x: f32 = time.sin() * radius;
		// let cam_z: f32 = time.cos() * radius;
		let model = math::rotate(&model, 45.0f32.to_radians(), &(0.5, 1.0, 0.0).into());

		// let view = math::matrix::Matrix4::new_identity();
		// let view = math::translate(&view, &((0.0, 0.0, -3.0).into()));
		let view = math::look_at(
			camera_pos,
			&(*camera_pos + *camera_front),
			camera_up
		);

		let projection = math::matrix::Matrix4::new_perspective(45.0f32.to_radians(), 900.0/700.0, 0.1, 100.0);
		// let projection = math::matrix::Matrix4::new_identity();

		// let transform_location = unsafe {
		// 	let string = CString::new("transform").unwrap();
		// 	gl::GetUniformLocation(self.program.id(), string.as_ptr())
		// };

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

		// println!("Model, view, projection: {} {} {}", model_location, view_location, projection_location);

		self.tex.activate(gl::TEXTURE0);
		self.program.set_used();


		unsafe
		{
			// Need to transpose the matrix before passing to the shader, as opengl expects numbers in columns, and we save numbers in rows
			// gl::UniformMatrix4fv(transform_location, 1, gl::FALSE, &transform.transposed() as *const math::matrix::Matrix4 as *const f32);
			gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &model.transposed() as *const math::matrix::Matrix4 as *const f32);
			gl::UniformMatrix4fv(view_location, 1, gl::FALSE, &view.transposed() as *const math::matrix::Matrix4 as *const f32);
			gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, &projection.transposed() as *const math::matrix::Matrix4 as *const f32);
		}
		self.tex.bind();
		self.vao.bind();

		unsafe
		{
			// gl::DrawElements(
			// 	gl::TRIANGLES,
			// 	6,
			// 	gl::UNSIGNED_INT,
			// 	0 as *const gl::types::GLvoid
			// )
			gl::DrawArrays(gl::TRIANGLES, 0, 36);
		}

		// let transform = math::matrix::Matrix4::new_identity();
		// let v = math::vector::Vector3::new(-0.5, 0.5, 0.0);
		// let transform = math::translate(&transform, &v);
		// // let v = math::vector::Vector3::new(0.0, 0.0, 1.0);
		// // let transform = math::rotate(&transform, (angle / 10.0).to_radians(), &v);
		// let v = math::vector::Vector3::new(0.5, 0.5, 0.0);
		// let transform = math::scale(&transform, &v);

		// let transform_location = unsafe {
		// 	let string = CString::new("transform").unwrap();
		// 	gl::GetUniformLocation(self.program.id(), string.as_ptr())
		// };

		// self.tex.activate(gl::TEXTURE0);
		// self.program.set_used();


		// unsafe
		// {
		// 	// Need to transpose the matrix before passing to the shader, as opengl expects numbers in columns, and we save numbers in rows
		// 	gl::UniformMatrix4fv(transform_location, 1, gl::FALSE, &transform.transposed() as *const math::matrix::Matrix4 as *const f32);
		// }

		// unsafe
		// {
		// 	gl::DrawElements(
		// 		gl::TRIANGLES,
		// 		6,
		// 		gl::UNSIGNED_INT,
		// 		0 as *const gl::types::GLvoid
		// 	)
		// }
	}
}