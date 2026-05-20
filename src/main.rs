mod vec2;
mod vec3;

use crate::vec2::Vec2;
use crate::vec3::Vec3;
use sdl3::{event::Event, pixels::Color};
use std::time::{Duration, Instant};

const TICK_RATE: f64 = 120_f64;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

struct Mesh {
	vertices: Vec<Vec3<f32>>,
	indices: Vec<[u32; 3]>,
}

impl Mesh {
	fn new(vertices: Vec<Vec3<f32>>, indices: Vec<[u32; 3]>) -> Self {
		Self {
			vertices: vertices,
			indices: indices,
		}
	}
}

fn main() {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context
		.video()
		.unwrap();

	let window = video_subsystem
		.window("gebbit", WINDOW_WIDTH, WINDOW_HEIGHT)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas();

	let mut event_pump = sdl_context
		.event_pump()
		.unwrap();

	let vertices = vec![
		Vec3::new(-0.5, -0.5, -0.5),
		Vec3::new(0.5, -0.5, -0.5),
		Vec3::new(0.5, 0.5, -0.5),
		Vec3::new(-0.5, 0.5, -0.5),
		Vec3::new(-0.5, -0.5, 0.5),
		Vec3::new(0.5, -0.5, 0.5),
		Vec3::new(0.5, 0.5, 0.5),
		Vec3::new(-0.5, 0.5, 0.5),
	];

	let indices = vec![
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

	let mut last_frame = Instant::now();
	let mut accumulator = Duration::new(0, 0);
	let tick_time = Duration::from_secs_f64(1_f64 / TICK_RATE);

	'running: loop {
		let now = Instant::now();
		accumulator += now.duration_since(last_frame);
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
			accumulator -= tick_time;
		}

		canvas.set_draw_color(Color::BLACK);
		canvas.clear();

		canvas.present();
	}
}
