use crate::{mesh::Mesh, vec3::Vec3};

pub struct Object {
	pub mesh: Mesh,
	pub pos: Vec3,
	pub rot: Vec3,
	pub l_vel: Vec3,
	pub a_vel: Vec3,
}

impl Object {
	pub fn new(mesh: Mesh) -> Self {
		Self {
			mesh: mesh,
			pos: Vec3::new(0_f32, 0_f32, 0_f32),
			rot: Vec3::new(0_f32, 0_f32, 0_f32),
			l_vel: Vec3::new(0_f32, 0_f32, 0_f32),
			a_vel: Vec3::new(0_f32, 0_f32, 0_f32),
		}
	}
}
