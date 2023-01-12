use core::{
	fmt,
	fmt::{Debug, Display, Formatter},
};
use embedded_graphics::pixelcolor::PixelColor;

#[derive(Debug)]
pub struct Image<'a, C: PixelColor> {
	pub(crate) widht: u16,
	pub(crate) colors: &'a [C],
}

#[derive(Debug)]
pub enum Dimension {
	Widht,
	Hight,
}

#[derive(Debug)]
pub enum Error {
	WrongPixelLength(Dimension),
}
impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::WrongPixelLength(dimension) => match dimension {
				Dimension::Widht => write!(f, "ength of `colors` is is not divisible by `widht`"),
				Dimension::Hight => write!(f, "`colors.len()/widht` does not macht provided `height`"),
			},
		}
	}
}
impl Error {
	/// for konst::result::unwrap_ctx; see https://docs.rs/konst/0.3.4/konst/result/macro.unwrap_ctx.html
	pub const fn panic(&self) -> ! {
		match self {
			Self::WrongPixelLength(dimension) => match dimension {
				Dimension::Widht => panic!("Error creating image:	length of `colors` is is not divisible by `widht`"),
				Dimension::Hight => panic!("Error creating image:	`colors.len()/widht` does not macht provided `height`"),
			},
		}
	}
}

impl<'a, C: PixelColor> Image<'a, C> {
	pub const fn new(colors: &'a [C], widht: u16, height: u16) -> Result<Self, Error> {
		if colors.len() % widht as usize != 0 {
			return Err(Error::WrongPixelLength(Dimension::Widht));
		};
		if colors.len() / widht as usize != height as usize {
			return Err(Error::WrongPixelLength(Dimension::Hight));
		};
		Ok(Image { widht, colors })
	}
}
