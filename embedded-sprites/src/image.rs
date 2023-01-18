use core::{
	fmt,
	fmt::{Debug, Display, Formatter},
};
use embedded_graphics::pixelcolor::PixelColor;

#[derive(Debug)]
pub struct Image<'a, C: PixelColor> {
	pub(crate) width: u16,
	pub(crate) transparenty: &'a [u8],
	pub(crate) colors: &'a [C],
}

#[derive(Debug, Eq, PartialEq)]
pub enum Dimension {
	Width,
	Height,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
	WrongPixelLength(Dimension),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::WrongPixelLength(dimension) => match dimension {
				Dimension::Width => write!(f, "length of `colors` is is not divisible by `width`"),
				Dimension::Height => write!(f, "`colors.len()/width` does not match provided `height`"),
			},
		}
	}
}
impl Error {
	/// for konst::result::unwrap_ctx; see https://docs.rs/konst/0.3.4/konst/result/macro.unwrap_ctx.html
	pub const fn panic(&self) -> ! {
		match self {
			Self::WrongPixelLength(dimension) => match dimension {
				Dimension::Width => panic!("Error creating image:	length of `colors` is is not divisible by `width`"),
				Dimension::Height => panic!("Error creating image:	`colors.len()/width` does not match provided `height`"),
			},
		}
	}
}

impl<'a, C: PixelColor> Image<'a, C> {
	pub const fn new(colors: &'a [C], transparenty: &'a [u8], width: u16, height: u16) -> Result<Self, Error> {
		if colors.len() % width as usize != 0 {
			return Err(Error::WrongPixelLength(Dimension::Width));
		};
		if colors.len() / width as usize != height as usize {
			return Err(Error::WrongPixelLength(Dimension::Height));
		};
		Ok(Image {
			colors,
			transparenty,
			width,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::{Dimension, Error, Image};
	use bitvec::{bitarr, prelude::*};
	use embedded_graphics::pixelcolor::Bgr565;
	use konst::result::unwrap_ctx;

	type Color = Bgr565;

	const IMAGE_DATA: [Color; 6] = [
		Color::new(255, 0, 0),
		Color::new(0, 255, 0),
		Color::new(0, 0, 255),
		Color::new(255, 0, 255),
		Color::new(255, 255, 255),
		Color::new(255, 255, 255),
	];

	#[test]
	fn create_const_image() {
		#[allow(dead_code)]
		const IMAGE1: Image<Color> = unwrap_ctx!(Image::new(&IMAGE_DATA, &bitarr![const 0,0,0,1,0,0], 3, 2));
		#[allow(dead_code)]
		const IMAGE2: Image<Color> = unwrap_ctx!(Image::new(&IMAGE_DATA, &bitarr![const 0,0,0,0], 3, 2));
		//todo: check if iterator of image is identical if I put them inside a sprite
	}
	#[test]
	fn create_image_wrong_widht() {
		assert_eq!(
			Image::new(&IMAGE_DATA, &bitarr![const 0,0,0,0], 4, 2).unwrap_err(),
			Error::WrongPixelLength(Dimension::Width)
		);
	}
	#[test]
	fn create_image_wrong_hight() {
		assert_eq!(
			Image::new(&IMAGE_DATA, &bitarr![const 0,0,0,0], 3, 3).unwrap_err(),
			Error::WrongPixelLength(Dimension::Height)
		);
	}
}
