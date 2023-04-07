use std::path::Path;
use image::EncodableLayout;

pub struct Texture
{
	id: gl::types::GLuint,
}

impl Texture
{
	pub fn new() -> Self
	{
		let mut tex: gl::types::GLuint = 0;
		unsafe
		{
			gl::GenTextures(1, &mut tex);

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::REPEAT as gl::types::GLint);
		}

		Texture { id: tex }
	}

	pub fn load(&self, path: &str)
	{
		self.bind();

		let img = image::open(Path::new(path)).unwrap().into_rgba8();

		unsafe
		{
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::RGBA as i32,
				img.width() as i32,
				img.height() as i32,
				0,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				img.as_bytes().as_ptr() as *const _
			);
			gl::GenerateMipmap(gl::TEXTURE_2D);
		}
	}

	pub fn set_wrapping(&self, mode: gl::types::GLuint)
	{
		self.bind();
		unsafe
		{
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, mode as gl::types::GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, mode as gl::types::GLint);
		}
	}

	pub fn set_filtering(&self, mode: gl::types::GLuint)
	{
		self.bind();
		unsafe
		{
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, mode as gl::types::GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mode as gl::types::GLint);
		}
	}

	pub fn bind(&self)
	{
		unsafe
		{
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}
	}

	pub fn activate(&self, unit: gl::types::GLuint)
	{
		unsafe
		{
			gl::ActiveTexture(unit);
			self.bind();
		}
	}
}

impl Drop for Texture
{
	fn drop(&mut self)
	{
		unsafe
		{
			gl::DeleteTextures(1, [self.id].as_ptr());
		}
	}
}