use crate::{camera::Camera, mesh::Mesh, object::Object, world::World};
use sdl3::{render::Canvas, video::Window};

pub struct Renderer {
	pub width: u32,
	pub height: u32,
}

impl Renderer {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
		}
	}

	fn render_mesh(&self, mesh: &Mesh, fov: f32, canvas: &mut Canvas<Window>) {
		let aspect = self.width as f32 / self.height as f32;
		let fov = fov.to_radians();

		for index in &mesh.indices {
			let v0 = mesh.vertices[index[0]];
			let v1 = mesh.vertices[index[1]];
			let v2 = mesh.vertices[index[2]];

			let p0 = v0.proj(fov, aspect);
			let p1 = v1.proj(fov, aspect);
			let p2 = v2.proj(fov, aspect);

			let s0 = p0.to_screen_space(self.width, self.height);
			let s1 = p1.to_screen_space(self.width, self.height);
			let s2 = p2.to_screen_space(self.width, self.height);

			canvas
				.draw_line(s0.as_point(), s1.as_point())
				.unwrap();
			canvas
				.draw_line(s1.as_point(), s2.as_point())
				.unwrap();
			canvas
				.draw_line(s2.as_point(), s0.as_point())
				.unwrap();
		}
	}

	fn render_object(&self, object: &Object, camera: &Camera, canvas: &mut Canvas<Window>) {
		let mut mesh = object.mesh.clone();

		mesh.vertices = mesh
			.vertices
			.iter()
			.map(|p| {
				p.rotate_xyz(object.rot)
					.add(object.pos)
			})
			.map(|p| {
				p.add(camera.pos.mul(-1_f32))
					.rotate_yxz(camera.rot.mul(-1_f32))
			})
			.collect();

		self.render_mesh(&mesh, camera.fov, canvas);
	}

	pub fn render_world(&self, world: &World, camera: &Camera, canvas: &mut Canvas<Window>) {
		for object in &world.objects {
			self.render_object(object, camera, canvas);
		}
	}
}
