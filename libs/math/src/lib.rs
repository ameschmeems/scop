pub mod vector;


#[cfg(test)]
mod tests {
	use super::*;
	use vector::{Vector2, Vector3};

    #[test]
	fn vec2_length()
	{
		let v = Vector2::new(3.0, 4.0);
		assert_eq!(v.length(), 5.0);
	}

	#[test]
	fn vec3_length()
	{
		let v = Vector3::new(4.0, 13.0, 16.0);
		assert_eq!(v.length(), 21.0);
	}

	#[test]
	fn vec2_eq()
	{
		let v = Vector2::new(1.0, 1.0);
		let w = Vector2::new(1.0, 1.0);
		let x = Vector2::new(2.0, 2.0);
		assert_eq!(v, w);
		assert_ne!(v, x);
		assert_ne!(w, x);
	}

	#[test]
	fn vec3_eq()
	{
		let v = Vector3::new(1.0, 1.0, 2.0);
		let w = Vector3::new(1.0, 1.0, 2.0);
		let x = Vector3::new(2.0, 2.0, 2.0);
		assert_eq!(v, w);
		assert_ne!(v, x);
		assert_ne!(w, x);
	}

	#[test]
	fn vec2_add()
	{
		let v = Vector2::new(1.0, 2.0);
		let w = Vector2::new(2.0, 3.0);
		let expected = Vector2::new(3.0, 5.0);
		assert_eq!(v + w, expected);
		assert_eq!(w + v, expected);
	}

	#[test]
	fn vec3_add()
	{
		let v = Vector3::new(1.0, 2.0, 3.0);
		let w = Vector3::new(2.0, 3.0, 5.0);
		let expected = Vector3::new(3.0, 5.0, 8.0);
		assert_eq!(v + w, expected);
		assert_eq!(w + v, expected);
	}

	#[test]
	fn vec2_scal_add()
	{
		let v = Vector2::new(2.0, 3.0);
		let expect = Vector2::new(4.0, 5.0);
		let scalar = 2.0;

		assert_eq!(v + scalar, expect);
		assert_eq!(scalar + v, expect);
	}

	#[test]
	fn vec3_scal_add()
	{
		let v = Vector3::new(2.0, 3.0, 1.0);
		let expect = Vector3::new(4.0, 5.0, 3.0);
		let scalar = 2.0;

		assert_eq!(v + scalar, expect);
		assert_eq!(scalar + v, expect);
	}

	#[test]
	fn vec2_sub()
	{
		let v = Vector2::new(2.0, 3.0);
		let w = Vector2::new(1.0, 1.0);
		let expect1 = Vector2::new(1.0, 2.0);
		let expect2 = Vector2::new(-1.0, -2.0);

		assert_eq!(v - w, expect1);
		assert_eq!(w - v, expect2);
	}

	#[test]
	fn vec3_sub()
	{
		let v = Vector3::new(2.0, 3.0, 5.0);
		let w = Vector3::new(1.0, 1.0, 2.0);
		let expect1 = Vector3::new(1.0, 2.0, 3.0);
		let expect2 = Vector3::new(-1.0, -2.0, -3.0);

		assert_eq!(v - w, expect1);
		assert_eq!(w - v, expect2);
	}

	#[test]
	fn vec2_scal_sub()
	{
		let v = Vector2::new(3.0, 2.0);
		let expect = Vector2::new(2.0, 1.0);
		let scalar = 1.0;

		assert_eq!(v - scalar, expect);
	}

	#[test]
	fn vec3_scal_sub()
	{
		let v = Vector3::new(3.0, 2.0, 5.0);
		let expect = Vector3::new(2.0, 1.0, 4.0);
		let scalar = 1.0;

		assert_eq!(v - scalar, expect);
	}

	#[test]
	fn vec2_scal_mul()
	{
		let v = Vector2::new(2.0, 4.0);
		let expect = Vector2::new(4.0, 8.0);
		let scalar = 2.0;

		assert_eq!(v * scalar, expect);
		assert_eq!(scalar * v, expect);
	}

	#[test]
	fn vec3_scal_mul()
	{
		let v = Vector3::new(2.0, 4.0, 3.0);
		let expect = Vector3::new(4.0, 8.0, 6.0);
		let scalar = 2.0;

		assert_eq!(v * scalar, expect);
		assert_eq!(scalar * v, expect);
	}

	#[test]
	fn vec2_scal_div()
	{
		let v = Vector2::new(3.0, 9.0);
		let expect = Vector2::new(1.0, 3.0);
		let scalar = 3.0;

		assert_eq!(v / scalar, expect);
	}

	#[test]
	fn vec3_scal_div()
	{
		let v = Vector3::new(3.0, 9.0, 6.0);
		let expect = Vector3::new(1.0, 3.0, 2.0);
		let scalar = 3.0;

		assert_eq!(v / scalar, expect);
	}

	#[test]
	fn vec2_neg()
	{
		let v = Vector2::new(5.0, 8.0);
		let expect = Vector2::new(-5.0, -8.0);

		assert_eq!(-v, expect);
		assert_eq!(-expect, v);
	}

	#[test]
	fn vec3_neg()
	{
		let v = Vector3::new(5.0, 8.0, 4.0);
		let expect = Vector3::new(-5.0, -8.0, -4.0);

		assert_eq!(-v, expect);
		assert_eq!(-expect, v);
	}

	#[test]
	fn vec2_dot()
	{
		let v = Vector2::new(0.6, -0.8);
		let w = Vector2::new(0.0, 1.0);
		let result = -0.8;

		assert_eq!(v.dot(&w), result);
		assert_eq!(w.dot(&v), result);
	}

	#[test]
	fn vec3_dot()
	{
		let v = Vector3::new(0.6, -0.8, 2.0);
		let w = Vector3::new(0.0, 1.0, 1.0);
		let result = 1.2;

		assert_eq!(v.dot(&w), result);
		assert_eq!(w.dot(&v), result);
	}

	#[test]
	fn vec2_norm()
	{
		let v = Vector2::new(0.0, 1.0);
		let w = Vector2::new(0.0, 3.0);
		let x = Vector2::new(0.0, 8.0);
		let y = Vector2::new(1.0, 0.0);
		let z = Vector2::new(17.0, 0.0);

		assert_eq!(v.normalized(), v);
		assert_eq!(w.normalized(), v);
		assert_eq!(x.normalized(), v);
		assert_eq!(y.normalized(), y);
		assert_eq!(z.normalized(), y);
	}

	#[test]
	fn vec3_norm()
	{
		let v = Vector3::new(0.0, 1.0, 0.0);
		let w = Vector3::new(0.0, 3.0, 0.0);
		let x = Vector3::new(0.0, 8.0, 0.0);
		let y = Vector3::new(1.0, 0.0, 0.0);
		let z = Vector3::new(17.0, 0.0, 0.0);
		let a = Vector3::new(0.0, 0.0, 1.0);
		let b = Vector3::new(0.0, 0.0, 23.0);

		assert_eq!(v.normalized(), v);
		assert_eq!(w.normalized(), v);
		assert_eq!(x.normalized(), v);
		assert_eq!(y.normalized(), y);
		assert_eq!(z.normalized(), y);
		assert_eq!(a.normalized(), a);
		assert_eq!(b.normalized(), a);
	}

	#[test]
	#[should_panic]
	fn vec3_invalid_cross_product()
	{
		let v = Vector3::new(1.0, 0.0, 0.0);
		let w = Vector3::new(12.0, 0.0, 0.0);

		v.cross(&w);
	}

	#[test]
	fn vec3_cross_product()
	{
		let v = Vector3::new(1.0, 0.0, 0.0);
		let w = Vector3::new(0.0, 1.0, 0.0);
		let expect = Vector3::new(0.0, 0.0, 1.0);

		assert_eq!(v.cross(&w), expect);
	}
}
