pub mod vector;
pub mod matrix;

use crate::vector::Vector3;
use crate::matrix::Matrix4;

pub fn translate(m: &Matrix4, v: &Vector3) -> Matrix4
{
	let m2 = Matrix4::new(
		(1.0, 0.0, 0.0, v.x()).into(),
		(0.0, 1.0, 0.0, v.y()).into(),
		(0.0, 0.0, 1.0, v.z()).into(),
		(0.0, 0.0, 0.0, 1.0).into()
	);

	*m * m2
}

pub fn rotate(m: &Matrix4, angle: f32, v: &Vector3) -> Matrix4
{
	let cosine = angle.cos();
	let sine = angle.sin();
	let axis = v.normalized();

	let rot1_1 = cosine + axis.x().powi(2) * (1.0 - cosine);
	let rot1_2 = axis.x() * axis.y() * (1.0 - cosine) - axis.z() * sine;
	let rot1_3 = axis.x() * axis.z() * (1.0 - cosine) + axis.y() * sine;
	let rot2_1 = axis.y() * axis.x() * (1.0 - cosine) + axis.z() * sine;
	let rot2_2 = cosine + axis.y().powi(2) * (1.0 - cosine);
	let rot2_3 = axis.y() * axis.z() * (1.0 - cosine) - axis.x() * sine;
	let rot3_1 = axis.z() * axis.x() * (1.0 - cosine) - axis.y() * sine;
	let rot3_2 = axis.z() * axis.y() * (1.0 - cosine) + axis.x() * sine;
	let rot3_3 = cosine + axis.z().powi(2) * (1.0 - cosine);

	let rot = Matrix4::new(
		(rot1_1, rot1_2, rot1_3, 0.0).into(),
		(rot2_1, rot2_2, rot2_3, 0.0).into(),
		(rot3_1, rot3_2, rot3_3, 0.0).into(),
		(0.0, 0.0, 0.0, 1.0).into()
	);

	*m * rot
}

pub fn scale(m: &Matrix4, v: &Vector3) -> Matrix4
{
	let scaling_matrix = Matrix4::new(
		(v.x(), 0.0, 0.0, 0.0).into(),
		(0.0, v.y(), 0.0, 0.0).into(),
		(0.0, 0.0, v.z(), 0.0).into(),
		(0.0, 0.0, 0.0, 1.0).into()
	);

	*m * scaling_matrix
}

#[cfg(test)]
mod tests {
	use super::*;
	use vector::{Vector2, Vector3, Vector4};
	use matrix::{Matrix2, Matrix3, Matrix4};

	extern crate nalgebra_glm as glm;

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

	#[test]
	fn mat2_add()
	{
		let m1 = Matrix2::new((1.0, 2.0).into(), (3.0, 4.0).into());
		let result = Matrix2::new((2.0, 4.0).into(), (6.0, 8.0).into());

		assert_eq!(m1 + m1, result);
	}

	#[test]
	fn mat3_add()
	{
		let m1 = Matrix3::new(
			(1.0, 2.0, 3.0).into(),
			(4.0, 5.0, 6.0).into(),
			(7.0, 8.0, 9.0).into()
		);

		let result = Matrix3::new(
			(2.0, 4.0, 6.0).into(),
			(8.0, 10.0, 12.0).into(),
			(14.0, 16.0, 18.0).into()
		);

		assert_eq!(m1 + m1, result);
	}

	#[test]
	fn mat4_add()
	{
		let m1 = Matrix4::new(
			(1.0, 2.0, 3.0, 4.0).into(),
			(2.0, 3.0, 4.0, 5.0).into(),
			(3.0, 4.0, 5.0, 6.0).into(),
			(4.0, 5.0, 6.0, 7.0).into()
		);

		let result = Matrix4::new(
			(2.0, 4.0, 6.0, 8.0).into(),
			(4.0, 6.0, 8.0, 10.0).into(),
			(6.0, 8.0, 10.0, 12.0).into(),
			(8.0, 10.0, 12.0, 14.0).into()
		);

		assert_eq!(m1 + m1, result);
	}

	#[test]
	fn mat2_sub()
	{
		let m1 = Matrix2::new((2.0, 2.0).into(), (3.0, 3.0).into());
		let m2 = Matrix2::new((1.0, 2.0).into(), (3.0, 4.0).into());
		let result = Matrix2::new((1.0, 0.0).into(), (0.0, -1.0).into());

		assert_eq!(m1 - m2, result);
	}

	#[test]
	fn mat3_sub()
	{
		let m1 = Matrix3::new(
			(1.0, 2.0, 3.0).into(),
			(4.0, 5.0, 6.0).into(),
			(7.0, 8.0, 9.0).into()
		);
		let m2 = Matrix3::new(
			(9.0, 8.0, 7.0).into(),
			(6.0, 5.0, 4.0).into(),
			(3.0, 2.0, 1.0).into()
		);
		let result = Matrix3::new(
			(-8.0, -6.0, -4.0).into(),
			(-2.0, 0.0, 2.0).into(),
			(4.0, 6.0, 8.0).into()
		);

		assert_eq!(m1 - m2, result);
	}

	#[test]
	fn mat4_sub()
	{
		let m1 = Matrix4::new(
			(1.0, 2.0, 3.0, 4.0).into(),
			(4.0, 3.0, 2.0, 1.0).into(),
			(2.0, 1.0, 4.0, 3.0).into(),
			(3.0, 4.0, 1.0, 2.0).into()
		);

		let m2 = Matrix4::new_identity();

		let result = Matrix4::new(
			(0.0, 2.0, 3.0, 4.0).into(),
			(4.0, 2.0, 2.0, 1.0).into(),
			(2.0, 1.0, 3.0, 3.0).into(),
			(3.0, 4.0, 1.0, 1.0).into()
		);

		assert_eq!(m1 - m2, result);
	}

	#[test]
	fn mat2_mul()
	{
		let m1 = Matrix2::new((1.0, 2.0).into(), (3.0, 4.0).into());
		let m2 = Matrix2::new((5.0, 6.0).into(), (7.0, 8.0).into());
		let result = Matrix2::new((19.0, 22.0).into(), (43.0, 50.0).into());

		assert_eq!(m1 * m2, result);
	}

	#[test]
	fn mat3_mul()
	{
		let m1 = Matrix3::new(
			(4.0, 2.0, 0.0).into(),
			(0.0, 8.0, 1.0).into(),
			(0.0, 1.0, 0.0).into()
		);

		let m2 = Matrix3::new(
			(4.0, 2.0, 1.0).into(),
			(2.0, 0.0, 4.0).into(),
			(9.0, 4.0, 2.0).into()
		);

		let result = Matrix3::new(
			(20.0, 8.0, 12.0).into(),
			(25.0, 4.0, 34.0).into(),
			(2.0, 0.0, 4.0).into()
		);

		assert_eq!(m1 * m2, result);
	}

	#[test]
	fn mat4_mul()
	{
		let m1 = Matrix4::new(
			(1.0, 1.0, 1.0, 1.0).into(),
			(2.0, 2.0, 2.0, 2.0).into(),
			(3.0, 3.0, 3.0, 3.0).into(),
			(4.0, 4.0, 4.0, 4.0).into()
		);
		let m2 = Matrix4::new_identity();
		assert_eq!(m1 * m2, m1);
	}

	#[test]
	fn mat2_mul_vec2()
	{
		let m = Matrix2::new((1.0, 2.0).into(), (3.0, 4.0).into());
		let v = Vector2::new(1.0, 2.0);
		let result = Vector2::new(5.0, 11.0);

		assert_eq!(m * v, result);
	}

	#[test]
	fn mat3_mul_vec3()
	{
		let m = Matrix3::new(
			(1.0, 1.0, 1.0).into(),
			(1.0, 1.0, 1.0).into(),
			(1.0, 1.0, 1.0).into()
		);

		let v = Vector3::new(1.0, 2.0, 3.0);

		let result = Vector3::new(6.0, 6.0, 6.0);

		assert_eq!(m * v, result);
	}

	#[test]
	fn mat4_mul_vec4()
	{
		let m = Matrix4::new(
			(1.0, 1.0, 1.0, 1.0).into(),
			(1.0, 1.0, 1.0, 1.0).into(),
			(1.0, 1.0, 1.0, 1.0).into(),
			(1.0, 1.0, 1.0, 1.0).into()
		);

		let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
		let result = Vector4::new(10.0, 10.0, 10.0, 10.0);

		assert_eq!(m * v, result);
	}

	#[test]
	fn translate()
	{
		let m = Matrix4::new_identity();
		let v = Vector3::new(1.0, 2.0, 3.0);
		let result = Matrix4::new(
			(1.0, 0.0, 0.0, 1.0).into(),
			(0.0, 1.0, 0.0, 2.0).into(),
			(0.0, 0.0, 1.0, 3.0).into(),
			(0.0, 0.0, 0.0, 1.0).into()
		);

		assert_eq!(crate::translate(&m, &v), result);
	}

	#[test]
	fn vec4_is_tightly_packed()
	{
		let v = Vector4::new(1.0, 2.0, 3.0, 4.0);

		// Cast the struct to an f32 pointer
		let ptr = &v as *const Vector4 as *const f32;

		let x = unsafe { *ptr };
		let y = unsafe { *ptr.add(1) };
		let z = unsafe { *ptr.add(2) };
		let w = unsafe { *ptr.add(3) };

		// println!("{x}");

		assert_eq!(x, v.x());
		assert_eq!(y, v.y());
		assert_eq!(z, v.z());
		assert_eq!(w, v.w());
	}

	#[test]
	fn mat4_is_tightly_packed()
	{
		let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
		let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
		let v3 = Vector4::new(9.0, 10.0, 11.0, 12.0);
		let v4 = Vector4::new(13.0, 14.0, 15.0, 16.0);
		let m = Matrix4::new(v1, v2, v3, v4);

		let ptr = &m as *const Matrix4 as *const Vector4;

		let t1 = unsafe { *ptr };
		let t2 = unsafe { *ptr.add(1) };
		let t3 = unsafe { *ptr.add(2) };
		let t4 = unsafe { *ptr.add(3) };

		assert_eq!(t1, v1);
		assert_eq!(t2, v2);
		assert_eq!(t3, v3);
		assert_eq!(t4, v4);
	}
}
