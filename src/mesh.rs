use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Mesh {
	pub vertices: Vec<Vec3>,
	pub indices: Vec<[usize; 3]>,
}

impl Mesh {
	pub fn new(vertices: Vec<Vec3>, indices: Vec<[usize; 3]>) -> Self {
		Self {
			vertices,
			indices,
		}
	}
}
