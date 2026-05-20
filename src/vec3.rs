use crate::vec2::Vec2;
use std::ops::Div;

#[derive(Clone, Copy)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T> Vec3<T>
where
	T: Copy + Div<Output = T>,
{
	pub fn new(x: T, y: T, z: T) -> Self {
		Self {
			x: x,
			y: y,
			z: z,
		}
	}

	pub fn proj(self) -> Vec2<T> {
		let x = self.x / self.z;
		let y = self.y / self.z;

		Vec2::new(x, y)
	}
}
