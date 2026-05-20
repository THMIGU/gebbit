use crate::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self {
			x: x,
			y: y,
			z: z,
		}
	}

	pub fn proj(self, fov: f32, aspect: f32) -> Vec2 {
		if self.z <= 0_f32 {
			return Vec2::new(0_f32, 0_f32);
		}

		let f = 1_f32 / (fov / 2_f32).tan();

		let x = (self.x * f) / (self.z * aspect);
		let y = (self.y * f) / self.z;

		Vec2::new(x, y)
	}
}
