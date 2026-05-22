use sdl3::rect::Point;

#[derive(Clone, Copy)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Self {
		Self {
			x,
			y,
		}
	}

	pub fn to_screen_space(self, width: u32, height: u32) -> Vec2 {
		let x = (self.x + 1_f32) / 2_f32 * width as f32;
		let y = (1_f32 - self.y) / 2_f32 * height as f32;

		Vec2::new(x, y)
	}

	pub fn as_point(self) -> Point {
		Point::new(self.x as i32, self.y as i32)
	}
}
