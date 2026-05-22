use crate::{camera::Camera, object::Object};

pub struct World {
	pub objects: Vec<Object>,
	pub cameras: Vec<Camera>,
}

impl World {
	pub fn new() -> Self {
		Self {
			objects: Vec::new(),
			cameras: Vec::new(),
		}
	}
}
