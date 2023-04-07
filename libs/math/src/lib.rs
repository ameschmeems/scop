pub mod vector;

#[cfg(test)]
mod tests {
	use super::*;
	use vector::{Vector2, Vector3, Vector4};

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
	fn vec4_length()
	{
		let v = Vector4::new(1.0, 1.0, 1.0, 1.0);
		assert_eq!(v.length(), 2.0);
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
	fn vec4_eq()
	{
		let v = Vector4::new(1.0, 1.0, 1.0, 1.0);
		let w = Vector4::new(1.0, 1.0, 1.0, 1.0);
		let x = Vector4::new(1.0, 2.0, 1.0, 2.0);
		let y = Vector4::new(1.0, 2.0, 1.0, 2.0);

		assert_eq!(v, w);
		assert_eq!(w, v);
		assert_eq!(x, y);
		assert_ne!(v, x);
		assert_ne!(v, y);
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
	fn vec4_add()
	{
		let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
		let w = Vector4::new(1.0, 1.0, 1.0, 1.0);
		let expect = Vector4::new(2.0, 3.0, 4.0, 5.0);
		assert_eq!(v + w, expect);
		assert_eq!(w + v, expect);
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
	fn vec4_scal_add()
	{
		let v = Vector4::new(2.0, 3.0, 4.0, 5.0);
		let expect = Vector4::new(4.0, 5.0, 6.0, 7.0);
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
	fn vec4_sub()
	{
		let v = Vector4::new(2.0, 3.0, 4.0, 5.0);
		let w = Vector4::new(1.0, 1.0, 2.0, 2.0);
		let expect1 = Vector4::new(1.0, 2.0, 2.0, 3.0);
		let expect2 = Vector4::new(-1.0, -2.0, -2.0, -3.0);

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
	fn vec4_scal_sub()
	{
		let v = Vector4::new(3.0, 2.0, 4.0, 1.0);
		let expect = Vector4::new(2.0, 1.0, 3.0, 0.0);
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
	fn vec4_scal_mull()
	{
		let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
		let expect = Vector4::new(2.0, 4.0, 6.0, 8.0);
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
	fn vec4_scal_div()
	{
		let v = Vector4::new(3.0, 9.0, 6.0, 6.0);
		let expect = Vector4::new(1.0, 3.0, 2.0, 2.0);
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
	fn vec4_neg()
	{
		let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
		let expect = Vector4::new(-1.0, -2.0, -3.0, -4.0);

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
	fn vec4_dot()
	{
		let v = Vector4::new(0.6, -0.8, 2.0, 1.0);
		let w = Vector4::new(0.0, 1.0, 1.0, 1.0);
		let result = 2.2;

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
	fn vec4_norm()
	{
		let v = Vector4::new(0.0, 1.0, 0.0, 0.0);
		let w = Vector4::new(0.0, 3.0, 0.0, 0.0);
		let x = Vector4::new(0.0, 8.0, 0.0, 0.0);
		let y = Vector4::new(1.0, 0.0, 0.0, 0.0);
		let z = Vector4::new(17.0, 0.0, 0.0, 0.0);
		let a = Vector4::new(0.0, 0.0, 1.0, 0.0);
		let b = Vector4::new(0.0, 0.0, 23.0, 0.0);
		let c = Vector4::new(0.0, 0.0, 0.0, 1.0);
		let d = Vector4::new(0.0, 0.0, 0.0, 12.0);
		let e = Vector4::new(0.0, 0.0, 0.0, 5.0);

		assert_eq!(v.normalized(), v);
		assert_eq!(w.normalized(), v);
		assert_eq!(x.normalized(), v);
		assert_eq!(y.normalized(), y);
		assert_eq!(z.normalized(), y);
		assert_eq!(a.normalized(), a);
		assert_eq!(b.normalized(), a);
		assert_eq!(c.normalized(), c);
		assert_eq!(d.normalized(), c);
		assert_eq!(e.normalized(), c);
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

	#[test]
	fn vec3_from_vec2()
	{
		let v = Vector2::new(1.0, 2.0);
		let w: Vector3 = (v, 3.0).into();
		let expect = Vector3::new(1.0, 2.0, 3.0);

		assert_eq!(w, expect);
	}

	#[test]
	fn vec4_from_vec3()
	{
		let v = Vector3::new(1.0, 2.0, 3.0);
		let w: Vector4 = (v, 4.0).into();
		let expect = Vector4::new(1.0, 2.0, 3.0, 4.0);

		assert_eq!(w, expect);
	}
}
