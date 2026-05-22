use crate::{mesh::Mesh, vec3::Vec3};

#[derive(Clone)]
pub struct Object {
	pub mesh: Mesh,
	pub pos: Vec3,
	pub rot: Vec3,
}

impl Object {
	pub fn new(mesh: Mesh) -> Self {
		Self {
			mesh,
			pos: Vec3::new(0_f32, 0_f32, 0_f32),
			rot: Vec3::new(0_f32, 0_f32, 0_f32),
		}
	}
}
