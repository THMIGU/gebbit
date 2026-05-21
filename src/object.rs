use crate::{mesh::Mesh, vec3::Vec3};

pub struct Object {
	pub mesh: Mesh,
	pub pos: Vec3,
	pub vel: Vec3,
	pub rot: Vec3,
}
