use crate::vec3::Vec3;

pub struct Camera {
	pub pos: Vec3,
	pub rot: Vec3,
	pub fov: f32,
}

impl Camera {
	pub fn new(fov: f32) -> Self {
		Self {
			pos: Vec3::new(0_f32, 0_f32, 0_f32),
			rot: Vec3::new(0_f32, 0_f32, 0_f32),
			fov: fov,
		}
	}
}
