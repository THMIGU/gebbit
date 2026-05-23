mod camera;
mod game;
mod mesh;
mod object;
mod renderer;
mod vec2;
mod vec3;
mod world;

use crate::{
	camera::Camera, game::Game, mesh::Mesh, object::Object, renderer::Renderer, vec3::Vec3,
	world::World,
};
use sdl3::event::Event;
use std::time::{Duration, Instant};

const TICK_RATE: f64 = 120_f64;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const FOV: f32 = 70_f32;

fn main() {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("gebbit", WINDOW_WIDTH, WINDOW_HEIGHT)
		.position_centered()
		.build()
		.unwrap();

	sdl_context
		.mouse()
		.set_relative_mouse_mode(&window, true);

	let mut canvas = window.into_canvas();

	let mut event_pump = sdl_context
		.event_pump()
		.unwrap();

	let vertices: Vec<Vec3> = vec![
		Vec3::new(-0.5, -0.5, -0.5),
		Vec3::new(0.5, -0.5, -0.5),
		Vec3::new(0.5, 0.5, -0.5),
		Vec3::new(-0.5, 0.5, -0.5),
		Vec3::new(-0.5, -0.5, 0.5),
		Vec3::new(0.5, -0.5, 0.5),
		Vec3::new(0.5, 0.5, 0.5),
		Vec3::new(-0.5, 0.5, 0.5),
	];

	let indices: Vec<[usize; 3]> = vec![
		[0, 1, 2],
		[0, 2, 3],
		[1, 5, 6],
		[1, 6, 2],
		[5, 4, 7],
		[5, 7, 6],
		[4, 0, 3],
		[4, 3, 7],
		[2, 3, 7],
		[2, 7, 6],
		[1, 0, 4],
		[1, 4, 5],
	];

	let cube_mesh = Mesh::new(vertices, indices);
	let mut cube = Object::new(cube_mesh);
	cube.pos = Vec3::new(0_f32, 0_f32, 2_f32);
	cube.rot = Vec3::new(0_f32, 45_f32.to_radians(), 0_f32);

	let mut camera = Camera::new(FOV);
	camera.rot = Vec3::new(0_f32, 0_f32, 0_f32);

	let mut world = World::new();
	world.objects.push(cube);
	world.cameras.push(camera);

	let renderer = Renderer::new(WINDOW_WIDTH, WINDOW_HEIGHT);

	let mut game = Game::new(world, renderer);

	let mut last_frame = Instant::now();
	let mut accumulator = Duration::new(0, 0);
	let tick_time = Duration::from_secs_f64(1_f64 / TICK_RATE);

	let mut fps_timer = Duration::ZERO;
	let mut fps_frames = 0;
	let mut displayed_fps = 0_f64;

	'running: loop {
		let now = Instant::now();
		let frame_duration = now.duration_since(last_frame);
		accumulator += frame_duration;
		last_frame = now;

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {
					..
				} => break 'running,
				_ => {}
			}
		}

		while accumulator >= tick_time {
			let keyboard = event_pump.keyboard_state();
			let mouse = event_pump.relative_mouse_state();
			game.update(keyboard, mouse);
			accumulator -= tick_time;
		}

		game.render(&mut canvas);

		fps_timer += frame_duration;
		fps_frames += 1;

		if fps_timer > Duration::from_secs(1) {
			displayed_fps = fps_frames as f64 / fps_timer.as_secs_f64();

			fps_timer = Duration::ZERO;
			fps_frames = 0;
		}

		canvas
			.window_mut()
			.set_title(&format!("gebbit {:.0} FPS", displayed_fps))
			.unwrap();
	}
}
