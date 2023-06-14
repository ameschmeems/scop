extern crate sdl2;
extern crate gl;
// open gl docs locally -> cargo doc -p gl --no-deps --open

pub mod render_gl;
pub mod resources;
pub mod triangle;
pub mod square;
pub mod cube;
pub mod model;
pub mod scene;
pub mod camera;

use resources::Resources;
use std::path::Path;
use sdl2::keyboard::Keycode;

fn main()
{
	let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
	let gl_attr = video_subsystem.gl_attr();
	sdl.mouse().set_relative_mouse_mode(true);

	// let timer = sdl2::Sdl::timer(&sdl).unwrap();
	let timer = sdl.timer().unwrap();

	// set opengl to core profile
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	// set version to 4.5
	gl_attr.set_context_version(4, 5);

    let window = video_subsystem.window("Scop", 900, 700).opengl().resizable().build().unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

	let mut viewport = render_gl::Viewport::for_window(900, 700);

	// let indices = vec![0, 1, 3, 1, 2, 3];
	// let vertices: Vec<model::Vertex> = vec![
	// 	model::Vertex::new((0.5, 0.5, 0.0).into()),
	// 	model::Vertex::new((0.5, -0.5, 0.0).into()),
	// 	model::Vertex::new((-0.5, -0.5, 0.0).into()),
	// 	model::Vertex::new((-0.5, 0.5, 0.0).into()),
	// ];

	let program = self::render_gl::Program::from_res(&res, "shaders/triangle").unwrap();

	// let square_mesh = model::Mesh::new(vertices, indices, program);
	// let square_mesh = model::Mesh::from_file("assets/stuff/teapot.obj", program);
	// let square_mesh = model::Mesh::from_file("assets/stuff/square.obj", program);
	let mesh_42 = model::Mesh::from_file("assets/models/teapot2.obj", program);
	// let cube_mesh = model::Mesh::from_file("assets/stuff/teapot.obj", program);

    unsafe
	{
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

	// let triangle = triangle::Triangle::new(&res, "assets/textures/wall.jpg").unwrap();
	// let square = square::Square::new(&res, "assets/textures/wall.jpg").unwrap();
	// let cube = cube::Cube::new(&res, "assets/textures/wall.jpg").unwrap();

	viewport.set_used();

	unsafe
	{
		// gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
		gl::Enable(gl::DEPTH_TEST);
	}

	// let time = SystemTime::now();

	let mut camera_pos = math::vector::Vector3::new(0.0, 0.0, 15.0);
	let mut camera_front = math::vector::Vector3::new(0.0, 0.0, 0.0);
	let camera_up = math::vector::Vector3::new(0.0, 1.0, 0.0);

	let mut first_mouse: bool = true;

	let mut yaw: f32 = -90.0;
	let mut pitch: f32 = 0.0;

	let camera_target = math::vector::Vector3::new(0.0, 0.0, 0.0);

	// let mut direction = math::vector::Vector3::new(
	// 	yaw.to_radians().cos() * pitch.to_radians().cos(),
	// 	pitch.to_radians().sin(),
	// 	yaw.to_radians().sin() * pitch.to_radians().cos()
	// );

	let mut direction = (camera_pos - camera_target).normalized();

	// camera_front = direction.normalized();

	let mut delta_time: f32 = 0.0;
	let mut last_frame: f32 = 0.0;

	let mut last_mouse_x: f32= 450.0;
	let mut last_mouse_y: f32 = 350.0;

	// projection matrix for the scene
	let projection = math::matrix::Matrix4::new_perspective(45.0f32.to_radians(), 900.0/700.0, 0.1, 100.0);

	let camera = camera::Camera::new();

	let mut scene = scene::Scene::new(vec![mesh_42], projection, camera);

    'main: loop
    {
        for event in event_pump.poll_iter()
        {
            // handle user input
            match event
            {
                sdl2::event::Event::Quit { .. } => break 'main,
				sdl2::event::Event::Window {
					win_event: sdl2::event::WindowEvent::Resized(w, h),
					..
				} => {
					viewport.update_size(w, h);
					viewport.set_used();
				},
				sdl2::event::Event::KeyDown {
					keycode: Some(key),
					..
				} => {
					let camera_speed: f32 = 0.01 * delta_time;
					match key
					{
						Keycode::Escape => { break 'main }
						_ => { scene.camera.update_camera(delta_time, event) }
					}
				},
				// sdl2::event::Event::MouseMotion {
				// 	// timestamp,
				// 	// window_id,
				// 	// which,
				// 	// mousestate,
				// 	// x,
				// 	// y,
				// 	xrel,
				// 	yrel,
				// 	..
				// } => {

				// 	if first_mouse // initially set to true
				// 	{
				// 		last_mouse_x = xrel as f32;
				// 		last_mouse_y = yrel as f32;
				// 		first_mouse = false;
				// 	}
				// 	let x_offset: f32 = xrel as f32 - last_mouse_x;
				// 	// Reversed, since y-coordinates range from bottom to top
				// 	let y_offset: f32 = last_mouse_y - yrel as f32;
				// 	// last_mouse_x = xrel as f32;
				// 	// last_mouse_y = yrel as f32;

				// 	let sensitivity: f32 = 0.1;
				// 	let x_offset = x_offset as f32 * sensitivity;
				// 	let y_offset = y_offset as f32 * sensitivity;

				// 	yaw += x_offset;
				// 	pitch += y_offset;

				// 	if pitch > 89.0
				// 	{
				// 		pitch = 89.0;
				// 	}
				// 	if pitch < -89.0
				// 	{
				// 		pitch = 89.0;
				// 	}

				// 	direction = math::vector::Vector3::new(
				// 		yaw.to_radians().cos() * pitch.to_radians().cos(),
				// 		pitch.to_radians().sin(),
				// 		yaw.to_radians().sin() * pitch.to_radians().cos()
				// 	);

				// 	camera_front = direction.normalized();
				// },
                _ => { scene.camera.update_camera(delta_time, event) },
            }
        }

        unsafe
        {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

		// triangle.render();
		// square.render(rotation);
		let current_frame = timer.ticks() as f32;
		delta_time = current_frame - last_frame;
		last_frame = current_frame;
		// cube.render(current_frame, &camera_pos, &camera_front, &camera_up);

		let radius = 10.0;
		camera_pos = math::vector::Vector3::new(((current_frame * 0.001).sin()) * radius, 0.0, (current_frame * 0.001).cos() * radius);

		scene.draw(&camera_pos, &camera_front, &camera_up);

		// mesh_42.render(&camera_pos,  &camera_front, &camera_up);
		// cube_mesh.render(&camera_pos,  &camera_front, &camera_up);
		// square_mesh.render(&camera_pos,  &camera_front, &camera_up);

        window.gl_swap_window();
    }
}
