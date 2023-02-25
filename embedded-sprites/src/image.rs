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

/// Create a transparenty map, to be used at image.
/// Length need not match the image length, missing data will be interpretet as not transparent.
/// ```
/// use embedded_sprites::transparency;
/// let transparency = transparency![0, 0, 1, 0];
/// ```
///third pixel ist transparent, rest is not transparent
#[macro_export]
macro_rules! transparency {
	($($x:expr),*) => {{	//TODO: enter const here if inline const is stable https://github.com/rust-lang/rust/issues/76001
		const N: usize = [$($x),*].len();
		let mut t = [0u8; N / 8 + if N % 8 > 0 { 1 } else { 0 }];
		let mut i = 0;
		let mut j = 7;
		$(
			t[i] |= ($x & 1 ) << j;
			#[allow(unused_assignments)]
			if j == 0 {
				j = 7;
				i += 1;
			} else {
				j -= 1;
			}
		)*
		t
	}};
}

#[cfg(test)]
mod tests {
	use super::{Dimension, Error, Image};
	use crate::transparency;
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
		const TRANSPARENCY1: &[u8] = &transparency![0, 0, 0, 1, 0, 0];
		#[allow(dead_code)]
		const IMAGE1: Image<Color> = unwrap_ctx!(Image::new(&IMAGE_DATA, TRANSPARENCY1, 3, 2));

		const TRANSPARENCY2: &[u8] = &transparency![0, 0, 0, 0];
		#[allow(dead_code)]
		const IMAGE2: Image<Color> = unwrap_ctx!(Image::new(&IMAGE_DATA, TRANSPARENCY2, 3, 2));
		//TODO: check if iterator of image is identical if I put them inside a sprite
	}
	#[test]
	fn create_image_wrong_widht() {
		assert_eq!(
			Image::new(&IMAGE_DATA, &transparency![0, 0, 0, 0], 4, 2).unwrap_err(),
			Error::WrongPixelLength(Dimension::Width)
		);
	}
	#[test]
	fn create_image_wrong_hight() {
		assert_eq!(
			Image::new(&IMAGE_DATA, &transparency![0, 0, 0, 0], 3, 3).unwrap_err(),
			Error::WrongPixelLength(Dimension::Height)
		);
	}
}
