use crate::{renderer::Renderer, vec3::Vec3, world::World};
use sdl3::{
	keyboard::{KeyboardState, Scancode},
	mouse::RelativeMouseState,
	pixels::Color,
	render::Canvas,
	video::Window,
};

pub struct Game {
	pub world: World,
	pub renderer: Renderer,
	pub ticks: u64,
}

impl Game {
	pub fn new(world: World, renderer: Renderer) -> Self {
		Self {
			world,
			renderer,
			ticks: 0,
		}
	}

	pub fn update(&mut self, keyboard: KeyboardState<'_>, mouse: RelativeMouseState) {
		self.ticks += 1;

		let cube = &mut self.world.objects[0];
		let camera = &mut self.world.cameras[0];

		cube.rot = cube
			.rot
			.add(Vec3::new(0_f32, 1_f32.to_radians(), 0_f32));

		let rotation = Vec3::new(mouse.y() / 120_f32, mouse.x() / 120_f32, 0_f32);

		let yaw = camera.rot.y;
		let forward = Vec3::new(yaw.sin(), 0_f32, yaw.cos());
		let right = Vec3::new(forward.z, 0_f32, -forward.x);

		let mut movement = Vec3::new(0_f32, 0_f32, 0_f32);
		if keyboard.is_scancode_pressed(Scancode::W) {
			movement = movement.add(forward.mul(0.01));
		}
		if keyboard.is_scancode_pressed(Scancode::S) {
			movement = movement.sub(forward.mul(0.01));
		}

		if keyboard.is_scancode_pressed(Scancode::A) {
			movement = movement.sub(right.mul(0.01));
		}
		if keyboard.is_scancode_pressed(Scancode::D) {
			movement = movement.add(right.mul(0.01));
		}

		if keyboard.is_scancode_pressed(Scancode::Space) {
			movement.y += 0.01;
		}
		if keyboard.is_scancode_pressed(Scancode::LShift) {
			movement.y -= 0.01;
		}

		camera.pos = camera.pos.add(movement);
		camera.rot = camera.rot.add(rotation);
		camera.rot.x = camera
			.rot
			.x
			.clamp(-90_f32.to_radians(), 90_f32.to_radians());
	}

	pub fn render(&mut self, canvas: &mut Canvas<Window>) {
		canvas.set_draw_color(Color::BLACK);
		canvas.clear();

		canvas.set_draw_color(Color::WHITE);
		self.renderer
			.render_world(&self.world, &self.world.cameras[0], canvas);

		canvas.present();
	}
}
