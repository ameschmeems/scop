use crate::vector::Vector3;
use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C, packed)]
pub struct Vector4
{
	x: f32,
	y: f32,
	z: f32,
	w: f32
}

impl Vector4
{
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self
	{
		Self { x, y, z, w }
	}

	pub fn length(&self) -> f32
	{
		let sum_squares = self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2);
		sum_squares.sqrt()
	}

	pub fn x(&self) -> f32
	{
		self.x
	}

	pub fn y(&self) -> f32
	{
		self.y
	}

	pub fn z(&self) -> f32
	{
		self.z
	}

	pub fn w(&self) -> f32
	{
		self.w
	}

	pub fn dot(&self, other: &Vector4) -> f32
	{
		(self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
	}

	pub fn normalized(&self) -> Vector4
	{
		let len = self.length();
		*self / len
	}

	// I dont want to deal with this right now, hopefully won't be needed
	// pub fn cross(&self, other: &Vector4) -> Vector4
	// {
	// 	if self.normalized() == other.normalized()
	// 	{
	// 		panic!("invalid cross product - both vectors are parallel: {:?} and {:?}", self, other);
	// 	}
	// 	else
	// 	{
	// 	}
	// }
}

impl ops::Add<Vector4> for Vector4
{
	type Output = Vector4;

	fn add(self, rhs: Vector4) -> Vector4
	{
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
			w: self.w + rhs.w
		}
	}
}

impl ops::Add<f32> for Vector4
{
	type Output = Vector4;

	fn add(self, rhs: f32) -> Vector4
	{
		Self {
			x: self.x + rhs,
			y: self.y + rhs,
			z: self.z + rhs,
			w: self.w + rhs
		}
	}
}

impl ops::Add<Vector4> for f32
{
	type Output = Vector4;

	fn add(self, rhs: Vector4) -> Vector4
	{
		rhs + self
	}
}

impl ops::Sub<Vector4> for Vector4
{
	type Output = Vector4;

	fn sub(self, rhs: Vector4) -> Self
	{
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
			w: self.w - rhs.w
		}
	}
}

impl ops::Sub<f32> for Vector4
{
	type Output = Vector4;

	fn sub(self, rhs: f32) -> Self
	{
		Self {
			x: self.x - rhs,
			y: self.y - rhs,
			z: self.z - rhs,
			w: self.w - rhs
		}
	}
}

impl ops::Mul<Vector4> for Vector4
{
	type Output = Vector4;

	fn mul(self, rhs: Vector4) -> Self
	{
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			z: self.z * rhs.z,
			w: self.w * rhs.w
		}
	}
}

impl ops::Mul<f32> for Vector4
{
	type Output = Vector4;

	fn mul(self, rhs: f32) -> Self
	{
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
			w: self.w * rhs
		}
	}
}

impl ops::Mul<Vector4> for f32
{
	type Output = Vector4;

	fn mul (self, rhs: Vector4) -> Vector4
	{
		rhs * self
	}
}

impl ops::Div<f32> for Vector4
{
	type Output = Vector4;

	fn div(self, rhs: f32) -> Vector4
	{
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
			w: self.w / rhs
		}
	}
}

impl ops::Neg for Vector4
{
	type Output = Vector4;

	fn neg(self) -> Vector4
	{
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
			w: -self.w
		}
	}
}

impl From<(f32, f32, f32, f32)> for Vector4
{
	fn from(other: (f32, f32, f32, f32)) -> Self
	{
		Self {
			x: other.0,
			y: other.1,
			z: other.2,
			w: other.3
		}
	}
}

impl From<(Vector3, f32)> for Vector4
{
	fn from(other: (Vector3, f32)) -> Self
	{
		Self {
			x: other.0.x(),
			y: other.0.y(),
			z: other.0.z(),
			w: other.1
		}
	}
}
