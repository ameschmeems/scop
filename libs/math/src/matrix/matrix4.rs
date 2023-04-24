use std::ops;
use crate::vector::Vector4;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C, packed)]
pub struct Matrix4
{
	pub(crate) r1: Vector4,
	pub(crate) r2: Vector4,
	pub(crate) r3: Vector4,
	pub(crate) r4: Vector4,
}

impl Matrix4
{
	pub fn new(r1: Vector4, r2: Vector4, r3: Vector4, r4: Vector4) -> Self
	{
		Matrix4 {
			r1,
			r2,
			r3,
			r4
		}
	}

	// fov is in radians
	pub fn new_perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self
	{
		let temp: f32 = (fov / 2.0).tan();
		Matrix4 {
			r1: (1.0 / (aspect * temp), 0.0, 0.0, 0.0).into(),
			r2: (0.0, 1.0 / temp, 0.0, 0.0).into(),
			r3: (0.0, 0.0, -(far + near) / (far - near), -(2.0 * far * near) / (far - near)).into(),
			r4: (0.0, 0.0, -1.0, 0.0).into()
		}
	}

	pub fn new_empty() -> Self
	{
		Matrix4 {
			r1: (0.0, 0.0, 0.0, 0.0).into(),
			r2: (0.0, 0.0, 0.0, 0.0).into(),
			r3: (0.0, 0.0, 0.0, 0.0).into(),
			r4: (0.0, 0.0, 0.0, 0.0).into()
		}
	}

	pub fn new_identity() -> Self
	{
		Matrix4 {
			r1: (1.0, 0.0, 0.0, 0.0).into(),
			r2: (0.0, 1.0, 0.0, 0.0).into(),
			r3: (0.0, 0.0, 1.0, 0.0).into(),
			r4: (0.0, 0.0, 0.0, 1.0).into()
		}
	}

	pub fn transposed(&self) -> Self
	{
		Matrix4::new(
			(self.r1.x(), self.r2.x(), self.r3.x(), self.r4.x()).into(),
			(self.r1.y(), self.r2.y(), self.r3.y(), self.r4.y()).into(),
			(self.r1.z(), self.r2.z(), self.r3.z(), self.r4.z()).into(),
			(self.r1.w(), self.r2.w(), self.r3.w(), self.r4.w()).into()
		)
	}
}

impl ops::Add<Matrix4> for Matrix4
{
	type Output = Matrix4;

	fn add(self, rhs: Matrix4) -> Self
	{
		Matrix4 {
			r1: self.r1 + rhs.r1,
			r2: self.r2 + rhs.r2,
			r3: self.r3 + rhs.r3,
			r4: self.r4 + rhs.r4
		}
	}
}

impl ops::Sub<Matrix4> for Matrix4
{
	type Output = Matrix4;

	fn sub(self, rhs: Matrix4) -> Self
	{
		Matrix4 {
			r1: self.r1 - rhs.r1,
			r2: self.r2 - rhs.r2,
			r3: self.r3 - rhs.r3,
			r4: self.r4 - rhs.r4
		}
	}
}

impl ops::Mul<f32> for Matrix4
{
	type Output = Matrix4;

	fn mul(self, rhs: f32) -> Self
	{
		Matrix4 {
			r1: self.r1 * rhs,
			r2: self.r2 * rhs,
			r3: self.r3 * rhs,
			r4: self.r4 * rhs
		}
	}
}

impl ops::Mul<Matrix4> for Matrix4
{
	type Output = Matrix4;

	fn mul(self, rhs: Matrix4) -> Self
	{
		let res1_1 = (self.r1.x() * rhs.r1.x()) + (self.r1.y() * rhs.r2.x()) + (self.r1.z() * rhs.r3.x()) + (self.r1.w() * rhs.r4.x());
		let res1_2 = (self.r1.x() * rhs.r1.y()) + (self.r1.y() * rhs.r2.y()) + (self.r1.z() * rhs.r3.y()) + (self.r1.w() * rhs.r4.y());
		let res1_3 = (self.r1.x() * rhs.r1.z()) + (self.r1.y() * rhs.r2.z()) + (self.r1.z() * rhs.r3.z()) + (self.r1.w() * rhs.r4.z());
		let res1_4 = (self.r1.x() * rhs.r1.w()) + (self.r1.y() * rhs.r2.w()) + (self.r1.z() * rhs.r3.w()) + (self.r1.w() * rhs.r4.w());
		let res2_1 = (self.r2.x() * rhs.r1.x()) + (self.r2.y() * rhs.r2.x()) + (self.r2.z() * rhs.r3.x()) + (self.r2.w() * rhs.r4.x());
		let res2_2 = (self.r2.x() * rhs.r1.y()) + (self.r2.y() * rhs.r2.y()) + (self.r2.z() * rhs.r3.y()) + (self.r2.w() * rhs.r4.y());
		let res2_3 = (self.r2.x() * rhs.r1.z()) + (self.r2.y() * rhs.r2.z()) + (self.r2.z() * rhs.r3.z()) + (self.r2.w() * rhs.r4.z());
		let res2_4 = (self.r2.x() * rhs.r1.w()) + (self.r2.y() * rhs.r2.w()) + (self.r2.z() * rhs.r3.w()) + (self.r2.w() * rhs.r4.w());
		let res3_1 = (self.r3.x() * rhs.r1.x()) + (self.r3.y() * rhs.r2.x()) + (self.r3.z() * rhs.r3.x()) + (self.r3.w() * rhs.r4.x());
		let res3_2 = (self.r3.x() * rhs.r1.y()) + (self.r3.y() * rhs.r2.y()) + (self.r3.z() * rhs.r3.y()) + (self.r3.w() * rhs.r4.y());
		let res3_3 = (self.r3.x() * rhs.r1.z()) + (self.r3.y() * rhs.r2.z()) + (self.r3.z() * rhs.r3.z()) + (self.r3.w() * rhs.r4.z());
		let res3_4 = (self.r3.x() * rhs.r1.w()) + (self.r3.y() * rhs.r2.w()) + (self.r3.z() * rhs.r3.w()) + (self.r3.w() * rhs.r4.w());
		let res4_1 = (self.r4.x() * rhs.r1.x()) + (self.r4.y() * rhs.r2.x()) + (self.r4.z() * rhs.r3.x()) + (self.r4.w() * rhs.r4.x());
		let res4_2 = (self.r4.x() * rhs.r1.y()) + (self.r4.y() * rhs.r2.y()) + (self.r4.z() * rhs.r3.y()) + (self.r4.w() * rhs.r4.y());
		let res4_3 = (self.r4.x() * rhs.r1.z()) + (self.r4.y() * rhs.r2.z()) + (self.r4.z() * rhs.r3.z()) + (self.r4.w() * rhs.r4.z());
		let res4_4 = (self.r4.x() * rhs.r1.w()) + (self.r4.y() * rhs.r2.w()) + (self.r4.z() * rhs.r3.w()) + (self.r4.w() * rhs.r4.w());

		Matrix4 {
			r1: (res1_1, res1_2, res1_3, res1_4).into(),
			r2: (res2_1, res2_2, res2_3, res2_4).into(),
			r3: (res3_1, res3_2, res3_3, res3_4).into(),
			r4: (res4_1, res4_2, res4_3, res4_4).into()
		}
	}
}

impl ops::Mul<Vector4> for Matrix4
{
	type Output = Vector4;

	fn mul(self, rhs: Vector4) -> Vector4
	{
		Vector4::new(
			self.r1.x() * rhs.x() + self.r1.y() * rhs.y() + self.r1.z() * rhs.z() + self.r1.w() * rhs.w(),
			self.r2.x() * rhs.x() + self.r2.y() * rhs.y() + self.r2.z() * rhs.z() + self.r2.w() * rhs.w(),
			self.r3.x() * rhs.x() + self.r3.y() * rhs.y() + self.r3.z() * rhs.z() + self.r3.w() * rhs.w(),
			self.r4.x() * rhs.x() + self.r4.y() * rhs.y() + self.r4.z() * rhs.z() + self.r4.w() * rhs.w()
		)
	}
}