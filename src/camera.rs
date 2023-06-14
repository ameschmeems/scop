use math;
use sdl2;
use sdl2::keyboard::Keycode;

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

	pub fn update_camera(&mut self, delta_time: f32, event: sdl2::event::Event)
	{
		match event
		{
			sdl2::event::Event::KeyDown {
				keycode: Some(key),
				..
			} => {
				let camera_speed: f32 = 0.01;
				let radius = 10.0;
				match key
				{
					Keycode::A => {
						self.view = math::rotate(self.view(), 3_f32.to_radians(), &(0.0, -1.0, 0.0).into());
					},
					Keycode::D => {
						self.view = math::rotate(self.view(), 3_f32.to_radians(), &(0.0, 1.0, 0.0).into());
					},
					Keycode::W => {
						self.view = math::rotate(self.view(), 3_f32.to_radians(), &(-1.0, 0.0, 0.0).into());
					},
					Keycode::S => {
						self.view = math::rotate(self.view(), 3_f32.to_radians(), &(1.0, 0.0, 0.0).into());
					},
					_ => {}
				}
			},
			sdl2::event::Event::MouseMotion { 
				xrel,
				yrel,
				..
			} => {
				let sensitivity = 0.1;
				let x_offset = xrel as f32 * sensitivity;
				let y_offset = yrel as f32 * sensitivity;

				let identity = math::matrix::Matrix4::new_identity();
				let m1 = math::rotate(&identity, 3_f32.to_radians() * x_offset, &(0.0, 1.0, 0.0).into());
				let m2 = math::rotate(&identity, 3_f32.to_radians() * y_offset, &(1.0, 0.0, 0.0).into());
				let final_transform = m1 * m2;

				self.view = self.view * final_transform;

				println!("{} {}", x_offset, y_offset);
				// self.view = math::rotate(self.view(), 3_f32.to_radians() * y_offset, &(1.0, 0.0, 0.0).into());

				// self.view = math::rotate(self.view(), 3_f32.to_radians() * x_offset, &(0.0, 1.0, 0.0).into());
			},
			_ => {}
		}
	}
}