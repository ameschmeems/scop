// Constant generic makes this struct work regardles of what type of buffer we want to use
// (gl::ARRAY_BUFFER, gl::ELEMENT_ARRAY_BUFFER, etc.)

pub type ArrayBuffer = Buffer<{gl::ARRAY_BUFFER}>;
pub type ElementArrayBuffer = Buffer<{gl::ELEMENT_ARRAY_BUFFER}>;
pub struct Buffer<const BUFFER_TYPE: gl::types::GLenum>
{
	vbo: gl::types::GLuint,
}

impl<const BUFFER_TYPE: gl::types::GLenum> Buffer<BUFFER_TYPE>
{
	pub fn new() -> Self
	{
		let mut vbo: gl::types::GLuint = 0;
		// first argument specifies how many VBOs we want, the second is an array that will be overwritten with the VBOs created
		// rust references act as pointers here, so passing &vbo works as long as we only ask for 1 VBO
		// (yes, the 'unsafe' is here for that reason)
		unsafe
		{
			gl::GenBuffers(1, &mut vbo);
		}

		Self { vbo }
	}

	pub fn bind(&self)
	{
		unsafe
		{
			gl::BindBuffer(BUFFER_TYPE, self.vbo);
		}
	}

	pub fn unbind(&self)
	{
		unsafe
		{
			gl::BindBuffer(BUFFER_TYPE, 0);
		}
	}

	pub fn static_draw_data<T>(&self, data: &[T])
	{
		unsafe
		{
			gl::BufferData(
				BUFFER_TYPE,
				(data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
				data.as_ptr() as *const gl::types::GLvoid, // pointer to data
				gl::STATIC_DRAW
			);
		}
	}
}

impl<const BUFFER_TYPE: gl::types::GLenum> Drop for Buffer<BUFFER_TYPE>
{
	fn drop(&mut self)
	{
		unsafe
		{
			gl::DeleteBuffers(1, &mut self.vbo);
		}
	}
}

pub struct VertexArray
{
	vao: gl::types::GLuint,
}

impl VertexArray
{
	pub fn new() -> Self
	{
		let mut vao: gl::types::GLuint = 0;
		unsafe
		{
			gl::GenVertexArrays(1, &mut vao);
		}

		VertexArray { vao }
	}

	pub fn bind(&self)
	{
		unsafe
		{
			gl::BindVertexArray(self.vao);
		}
	}

	pub fn unbind(&self)
	{
		unsafe
		{
			gl::BindVertexArray(0);
		}
	}
}

impl Drop for VertexArray
{
	fn drop(&mut self)
	{
		unsafe
		{
			gl::DeleteVertexArrays(1, &mut self.vao);
		}
	}
}