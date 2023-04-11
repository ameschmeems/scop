use std::ops;
use crate::vector::Vector3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix3
{
	r1: Vector3,
	r2: Vector3,
	r3: Vector3
}

impl Matrix3
{
	pub fn new(r1: Vector3, r2: Vector3, r3: Vector3) -> Self
	{
		Matrix3 {
			r1,
			r2,
			r3
		}
	}

	pub fn new_empty() -> Self
	{
		Matrix3 {
			r1: (0.0, 0.0, 0.0).into(),
			r2: (0.0, 0.0, 0.0).into(),
			r3: (0.0, 0.0, 0.0).into()
		}
	}

	pub fn new_identity() -> Self
	{
		Matrix3 {
			r1: (1.0, 0.0, 0.0).into(),
			r2: (0.0, 1.0, 0.0).into(),
			r3: (0.0, 0.0, 1.0).into()
		}
	}
}

impl ops::Add<Matrix3> for Matrix3
{
	type Output = Matrix3;

	fn add(self, rhs: Matrix3) -> Self
	{
		Matrix3 {
			r1: self.r1 + rhs.r1,
			r2: self.r2 + rhs.r2,
			r3: self.r3 + rhs.r3
		}
	}
}

impl ops::Sub<Matrix3> for Matrix3
{
	type Output = Matrix3;

	fn sub(self, rhs: Matrix3) -> Self
	{
		Matrix3 {
			r1: self.r1 - rhs.r1,
			r2: self.r2 - rhs.r2,
			r3: self.r3 - rhs.r3
		}
	}
}

impl ops::Mul<f32> for Matrix3
{
	type Output = Matrix3;

	fn mul(self, rhs: f32) -> Self
	{
		Matrix3 {
			r1: self.r1 * rhs,
			r2: self.r2 * rhs,
			r3: self.r3 * rhs
		}
	}
}

impl ops::Mul<Matrix3> for Matrix3
{
	type Output = Matrix3;

	fn mul(self, rhs : Matrix3) -> Self
	{
		let res1_1 = (self.r1.x() * rhs.r1.x()) + (self.r1.y() * rhs.r2.x()) + (self.r1.z() * rhs.r3.x());
		let res1_2 = (self.r1.x() * rhs.r1.y()) + (self.r1.y() * rhs.r2.y()) + (self.r1.z() * rhs.r3.y());
		let res1_3 = (self.r1.x() * rhs.r1.z()) + (self.r1.y() * rhs.r2.z()) + (self.r1.z() * rhs.r3.z());
		let res2_1 = (self.r2.x() * rhs.r1.x()) + (self.r2.y() * rhs.r2.x()) + (self.r2.z() * rhs.r3.x());
		let res2_2 = (self.r2.x() * rhs.r1.y()) + (self.r2.y() * rhs.r2.y()) + (self.r2.z() * rhs.r3.y());
		let res2_3 = (self.r2.x() * rhs.r1.z()) + (self.r2.y() * rhs.r2.z()) + (self.r2.z() * rhs.r3.z());
		let res3_1 = (self.r3.x() * rhs.r1.x()) + (self.r3.y() * rhs.r2.x()) + (self.r3.z() * rhs.r3.x());
		let res3_2 = (self.r3.x() * rhs.r1.y()) + (self.r3.y() * rhs.r2.y()) + (self.r3.z() * rhs.r3.y());
		let res3_3 = (self.r3.x() * rhs.r1.z()) + (self.r3.y() * rhs.r2.z()) + (self.r3.z() * rhs.r3.z());

		Matrix3 {
			r1: (res1_1, res1_2, res1_3).into(),
			r2: (res2_1, res2_2, res2_3).into(),
			r3: (res3_1, res3_2, res3_3).into()
		}

	}
}