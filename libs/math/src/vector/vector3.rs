use crate::vector::Vector2;
use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3
{
	x: f32,
	y: f32,
	z: f32,
}

impl Vector3
{
	pub fn new(x: f32, y: f32, z: f32) -> Self
	{
		Self { x, y, z }
	}

	pub fn length(&self) -> f32
	{
		let sum_squares = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
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

	pub fn dot(&self, other: &Vector3) -> f32
	{
		(self.x * other.x) + (self.y * other.y) + (self.z * other.z)
	}

	pub fn normalized(&self) -> Vector3
	{
		let len = self.length();
		Self { x: self.x / len, y: self.y / len, z: self.z / len }
	}

	pub fn cross(&self, other: &Vector3) -> Vector3
	{
		if self.normalized() == other.normalized()
		{
			panic!("invalid cross product - both vectors are parallel: {:?} and {:?}", self, other);
		}
		else
		{
			Self {
				x: (self.y * other.z) - (self.z * other.y),
				y: (self.z * other.x) - (self.x * other.z),
				z: (self.x * other.y) - (self.y * other.x)
			}
		}
	}
}

impl ops::Add<Vector3> for Vector3
{
	type Output = Vector3;

	fn add(self, rhs: Vector3) -> Vector3
	{
		Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
	}
}

impl ops::Add<f32> for Vector3
{
	type Output = Vector3;

	fn add(self, rhs: f32) -> Vector3
	{
		Self { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs}
	}
}

impl ops:: Add<Vector3> for f32
{
	type Output = Vector3;

	fn add(self, rhs: Vector3) -> Vector3
	{
		rhs + self
	}
}

impl ops::Sub<Vector3> for Vector3
{
	type Output = Vector3;

	fn sub(self, rhs: Vector3) -> Vector3
	{
		Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
	}
}

impl ops::Sub<f32> for Vector3
{
	type Output = Vector3;

	fn sub(self, rhs: f32) -> Vector3
	{
		Self { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
	}
}

impl ops::Mul<f32> for Vector3
{
	type Output = Vector3;

	fn mul(self, rhs: f32) -> Vector3
	{
		Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
	}
}

impl ops::Mul<Vector3> for f32
{
	type Output = Vector3;

	fn mul(self, rhs: Vector3) -> Vector3
	{
		rhs * self
	}
}

impl ops::Div<f32> for Vector3
{
	type Output = Vector3;

	fn div(self, rhs: f32) -> Vector3
	{
		Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
	}
}

impl ops::Neg for Vector3
{
	type Output = Vector3;

	fn neg(self) -> Vector3
	{
		Self { x: -self.x, y: -self.y, z: -self.z }
	}
}

impl From <(f32, f32, f32)> for Vector3
{
	fn from(other: (f32, f32, f32)) -> Self
	{
		Self { x: other.0, y: other.1, z: other.2}
	}
}

impl From<(Vector2, f32)> for Vector3
{
	fn from(other: (Vector2, f32)) -> Self
	{
		Self { x: other.0.x(), y: other.0.y(), z: other.1 }
	}
}