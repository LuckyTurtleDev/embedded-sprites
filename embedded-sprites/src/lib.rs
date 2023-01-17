#![no_std]
pub mod image;
pub mod sprite;

pub use embedded_sprites_proc_macro::*;

#[doc(hidden)]
pub mod private {
	use embedded_graphics::{pixelcolor::Bgr888, prelude::RgbColor};

	pub trait Image {
		type Color: RgbColor;
	}

	impl<C: RgbColor> Image for crate::image::Image<'_, C> {
		type Color = C;
	}

	// Stolen from https://docs.rs/embedded-graphics-core/0.3.2/src/embedded_graphics_core/pixelcolor/conversion.rs.html#3-6
	pub const fn convert_channel(value: u8, from_max: u8, to_max: u8) -> u8 {
		((value as u16 * to_max as u16 + from_max as u16 / 2) / from_max as u16) as u8
	}

	pub const fn convert_from_bgr888<C: RgbColor>(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
		(
			convert_channel(r, Bgr888::MAX_R, C::MAX_R),
			convert_channel(g, Bgr888::MAX_G, C::MAX_G),
			convert_channel(b, Bgr888::MAX_B, C::MAX_B),
		)
	}
}
