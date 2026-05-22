use crate::{mesh::Mesh, object::Object};
use sdl3::{render::Canvas, video::Window};

pub struct Renderer {
	pub width: u32,
	pub height: u32,
	pub fov: f32,
}

impl Renderer {
	pub fn new(width: u32, height: u32, fov: f32) -> Self {
		Self {
			width: width,
			height: height,
			fov: fov,
		}
	}

	fn render_mesh(&self, mesh: &Mesh, canvas: &mut Canvas<Window>) {
		let aspect = self.width as f32 / self.height as f32;
		let fov = self
			.fov
			.to_radians();

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

	pub fn render_object(&self, object: &Object, canvas: &mut Canvas<Window>) {
		let mut mesh = object.mesh.clone();

		mesh.vertices = mesh
			.vertices
			.iter()
			.map(|p| {
				p.rotate(object.rot)
					.add(object.pos)
			})
			.collect();

		self.render_mesh(&mesh, canvas);
	}
}
