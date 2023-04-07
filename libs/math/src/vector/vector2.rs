use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2
{
	x: f32,
	y: f32,
}

impl Vector2
{
	pub fn new(x: f32, y: f32) -> Self
	{
		Self { x, y }
	}

	pub fn length(&self) -> f32
	{
		let sum_squares = self.x.powi(2) + self.y.powi(2);
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

	pub fn dot(&self, other: &Vector2) -> f32
	{
		(self.x * other.x) + (self.y * other.y)
	}

	pub fn normalized(&self) -> Vector2
	{
		let len = self.length();
		Self { x: self.x / len, y: self.y / len}
	}
}

impl ops::Add<Vector2> for Vector2
{
	type Output = Vector2;

	fn add(self, rhs: Vector2) -> Vector2
	{
		Self { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}

impl ops::Add<f32> for Vector2
{
	type Output = Vector2;

	fn add(self, rhs: f32) -> Vector2
	{
		Self { x: self.x + rhs, y: self.y + rhs }
	}
}

impl ops::Add<Vector2> for f32
{
	type Output = Vector2;

	fn add(self, rhs: Vector2) -> Vector2
	{
		Vector2 { x: rhs.x + self, y: rhs.y + self }
	}
}

impl ops::Sub<Vector2> for Vector2
{
	type Output = Vector2;

	fn sub(self, rhs: Vector2) -> Vector2
	{
		Self { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}

impl ops::Sub<f32> for Vector2
{
	type Output = Vector2;

	fn sub(self, rhs: f32) -> Vector2
	{
		Self { x: self.x - rhs, y: self.y - rhs }
	}
}

impl ops::Mul<f32> for Vector2
{
	type Output = Vector2;

	fn mul(self, rhs: f32) -> Vector2
	{
		Self { x: self.x * rhs, y: self.y * rhs }
	}
}

impl ops::Mul<Vector2> for f32
{
	type Output = Vector2;

	fn mul(self, rhs: Vector2) -> Vector2
	{
		rhs * self
	}
}

impl ops::Div<f32> for Vector2
{
	type Output = Vector2;

	fn div(self, rhs: f32) -> Vector2
	{
		Self { x: self.x / rhs, y: self.y / rhs }
	}
}

impl ops::Neg for Vector2
{
	type Output = Vector2;

	fn neg(self) -> Vector2
	{
		Self { x: -self.x, y: -self.y }
	}
}