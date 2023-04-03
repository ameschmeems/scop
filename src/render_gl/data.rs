#[derive(Copy, Clone, Debug)] // make deep copies of data, easily print data
#[repr(C, packed)] // C-like layout of memory, ensure no gaps between values
pub struct uint_uint_uint
{
	pub d0: gl::types::GLuint,
	pub d1: gl::types::GLuint,
	pub d2: gl::types::GLuint
}

impl uint_uint_uint
{
	pub fn new(d0: gl::types::GLuint, d1: gl::types::GLuint, d2: gl::types::GLuint) -> Self
	{
		uint_uint_uint { d0, d1, d2 }
	}
}

impl From<(gl::types::GLuint, gl::types::GLuint, gl::types::GLuint)> for uint_uint_uint
{
	fn from(other: (gl::types::GLuint, gl::types::GLuint, gl::types::GLuint)) -> Self
	{
		uint_uint_uint {
			d0: other.0,
			d1: other.1,
			d2: other.2
		}
	}
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]

pub struct f32_f32_f32
{
	pub d0: f32,
	pub d1: f32,
	pub d2: f32
}

impl f32_f32_f32
{
	pub fn new(d0: f32, d1: f32, d2: f32) -> Self
	{
		f32_f32_f32 { d0, d1, d2 }
	}

	pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize)
	{
		gl::EnableVertexAttribArray(location as gl::types::GLuint);
		gl::VertexAttribPointer(
			location as gl::types::GLuint, // index of the generic vertex attribute
			3, // Number of components per generic vertex attribute
			gl::FLOAT,
			gl::FALSE,
			stride as gl::types::GLint, // offset between consecutive attributes
			offset as *const gl::types::GLvoid // offset of the first component
		);
	}
}

// allow conversions from (f32, f32, f32) tuples
impl From<(f32, f32, f32)> for f32_f32_f32
{
	fn from(other: (f32, f32, f32)) -> Self
	{
		f32_f32_f32::new(other.0, other.1, other.2)
	}
}
