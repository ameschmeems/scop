use math::{self, look_at};
use sdl2;

pub struct Camera
{
	view: math::matrix::Matrix4,
	// event_pump: sdl2::EventPump,
	camera_pos: math::vector::Vector3,
	camera_front: math::vector::Vector3,
	camera_up: math::vector::Vector3,
}

impl Camera
{
	pub fn new() -> Self
	{
		let camera_pos = math::vector::Vector3::new(0.0, 0.0, 15.0);
		let camera_front = math::vector::Vector3::new(0.0, 0.0, 0.0);
		let camera_up = math::vector::Vector3::new(0.0, 1.0, 0.0);
		let view = math::look_at(
			&camera_pos,
			&(camera_front),
			&camera_up
		);
		Self {
			view,
			// &event_pump,
			camera_pos,
			camera_front,
			camera_up,
		}
	}

	pub fn view(&self) -> &math::matrix::Matrix4
	{
		&self.view
	}

	pub fn update_camera(&mut self, delta_time: f32, event: &sdl2::event::Event)
	{
		match event
		{
			// sdl2::event::Event::KeyDown {
			// 	keycode: Some(key),
			// 	..
			// } => {
			// 	let camera_speed: f32 = 0.01;
			// 	let radius = 10.0;
			// 	match key
			// 	{
			// 		Keycode::A => {
			// 			self.view = math::rotate(self.view(), 3_f32.to_radians(), &(0.0, -1.0, 0.0).into());
			// 		},
			// 		Keycode::D => {
			// 			self.view = math::rotate(self.view(), 3_f32.to_radians(), &(0.0, 1.0, 0.0).into());
			// 		},
			// 		Keycode::W => {
			// 			self.view = math::rotate(self.view(), 3_f32.to_radians(), &(-1.0, 0.0, 0.0).into());
			// 		},
			// 		Keycode::S => {
			// 			self.view = math::rotate(self.view(), 3_f32.to_radians(), &(1.0, 0.0, 0.0).into());
			// 		},
			// 		_ => {}
			// 	}
			// },
			sdl2::event::Event::MouseMotion { 
				xrel,
				yrel,
				..
			} => {
				let sensitivity = 0.1;
				
				let x_offset = *xrel as f32 * sensitivity;
				let y_offset = *yrel as f32 * sensitivity;

				let mut x = 1.0;
				let y = 1.0;

				if self.camera_pos.z() < 0.0
				{
					x = -x;
				}

				let identity = math::matrix::Matrix4::new_identity();
				let m1 = math::rotate(&identity, 3_f32.to_radians() * x_offset, &(0.0, y, 0.0).into());
				let m2 = math::rotate(&identity, 3_f32.to_radians() * y_offset, &(x, 0.0, 0.0).into());
				let final_transform = m1 * m2;

				let temp1: math::vector::Vector4 = (self.camera_pos, 1.0).into();
				let temp2: math::vector::Vector4 = final_transform * temp1;
				let current_pos = math::vector::Vector3::new(temp2.x(), temp2.y(), temp2.z());

				self.camera_pos = current_pos;
				self.view = math::look_at(
					&self.camera_pos,
					&self.camera_front,
					&self.camera_up
				);

				// self.view = math::rotate(self.view(), 3_f32.to_radians() * y_offset, &(1.0, 0.0, 0.0).into());
				// self.view = math::rotate(self.view(), 3_f32.to_radians() * x_offset, &(0.0, 1.0, 0.0).into());
			},
			_ => {}
		}
	}
}