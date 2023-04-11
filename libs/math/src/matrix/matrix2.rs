use std::ops;
use crate::vector::Vector2;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix2
{
	r1: Vector2,
	r2: Vector2
}

impl Matrix2
{
	pub fn new(r1: Vector2, r2: Vector2) -> Self
	{
		Matrix2 {
			r1,
			r2
		}
	}

	pub fn new_empty() -> Self
	{
		Matrix2 {
			r1: (0.0, 0.0).into(),
			r2: (0.0, 0.0).into()
		}
	}

	pub fn new_identity() -> Self
	{
		Matrix2 {
			r1: (1.0, 0.0).into(),
			r2: (0.0, 1.0).into()
		}
	}
}

impl ops::Add<Matrix2> for Matrix2
{
	type Output = Matrix2;

	fn add(self, rhs: Matrix2) -> Self
	{
		Matrix2 {
			r1: self.r1 + rhs.r1,
			r2: self.r2 + rhs.r2
		}
	}
}

impl ops::Sub<Matrix2> for Matrix2
{
	type Output = Matrix2;

	fn sub(self, rhs: Matrix2) -> Self
	{
		Matrix2 {
			r1: self.r1 - rhs.r1,
			r2: self.r2 - rhs.r2
		}
	}
}

impl ops::Mul<f32> for Matrix2
{
	type Output = Matrix2;

	fn mul(self, rhs: f32) -> Self
	{
		Matrix2 {
			r1: self.r1 * rhs,
			r2: self.r2 * rhs
		}
	}
}

impl ops::Mul<Matrix2> for Matrix2
{
	type Output = Matrix2;

	fn mul(self, rhs: Matrix2) -> Self
	{
		let res1_1 = (self.r1.x() * rhs.r1.x()) + (self.r1.y() * rhs.r2.x());
		let res1_2 = (self.r1.x() * rhs.r1.y()) + (self.r1.y() * rhs.r2.y());
		let res2_1 = (self.r2.x() * rhs.r1.x()) + (self.r2.y() * rhs.r2.x());
		let res2_2 = (self.r2.x() * rhs.r1.y()) + (self.r2.y() * rhs.r2.y());

		Matrix2 {
			r1: (res1_1, res1_2).into(),
			r2: (res2_1, res2_2).into()
		}
	}
}

impl ops::Mul<Vector2> for Matrix2
{
	type Output = Vector2;

	fn mul(self, rhs: Vector2) -> Vector2
	{
		Vector2::new(
			self.r1.x() * rhs.x() + self.r1.y() * rhs.y(),
			self.r2.x() * rhs.x() + self.r2.y() * rhs.y()
		)
	}
}