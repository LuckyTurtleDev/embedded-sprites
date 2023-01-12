use crate::image::Image;
use core::fmt::Debug;
use embedded_graphics::{geometry::Point, pixelcolor::PixelColor, prelude::DrawTarget, Drawable, Pixel};

#[derive(Debug)]
pub struct Sprite<'a, C: PixelColor> {
	//postion of the top left pixel of the Sprite
	pub offset: Point,
	pub image: &'a Image<'a, C>,
}

impl<'a, C: PixelColor> Sprite<'a, C> {
	pub const fn new(offset: Point, image: &'a Image<C>) -> Self {
		Sprite { offset, image }
	}
}

#[derive(Debug)]
pub struct PixelIter<'a, C: PixelColor> {
	/// index of the next elment
	next: usize,
	sprite: &'a Sprite<'a, C>,
}

impl<'a, C: PixelColor> Iterator for PixelIter<'a, C> {
	type Item = Pixel<C>;
	fn next(&mut self) -> Option<Self::Item> {
		if self.next < self.sprite.image.colors.len() {
			let color = self.sprite.image.colors[self.next];
			let point = Point::new(
				(self.next % self.sprite.image.widht as usize) as i32 + self.sprite.offset.x,
				(self.next / self.sprite.image.widht as usize) as i32 + self.sprite.offset.y,
			);
			self.next += 1;
			return Some(Pixel(point, color));
		}
		None
	}
}

impl<'a, C: PixelColor> Drawable for Sprite<'a, C> {
	type Color = C;
	type Output = ();

	fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
	where
		D: DrawTarget<Color = Self::Color>,
	{
		target.draw_iter(PixelIter { next: 0, sprite: self })
	}
}
