#![no_std]

//! Embedded no std graphics library for bundling image at compile time, to be used at the [embedded-graphics](https://crates.io/crates/embedded-graphics) crate.
//!
//! The [`include_image`](crate::include_image) macro can be usede to create a [`Image`](crate::image::Image) from an existing image file at compile time.
//! Every image formats supported by the [image crate](https://crates.io/crates/image) can be used.
//! The image will be automatically be converted to the requested pixelcolor.
//! Current only rgb pixelcolors are supported.
//! ```
//! use embedded_graphics::pixelcolor::Bgr565;
//! use embedded_sprites::{image::Image, include_image};
//! #[include_image]
//! const IMAGE: Image<Bgr565> = "embedded-sprites/grass.png";
//! ```
//!
//! To draw a [`Image`](crate::image::Image) it must be put inside a [`Sprite`](crate::sprite::Sprite).
//! You can use the same [`Image`](crate::image::Image) inside multiple [`Sprite`](crate::sprite::Sprite)s;
//! ```
//! # use embedded_graphics::prelude::Size;
//! # use embedded_sprites::{image::Image, include_image};
//! # use embedded_graphics_simulator::SimulatorDisplay;
//! use embedded_graphics::{geometry::Point, pixelcolor::Bgr565, Drawable};
//! use embedded_sprites::sprite::Sprite;
//!
//! # #[include_image]
//! # const IMAGE: Image<Bgr565> = "embedded-sprites/grass.png";
//! const SPRITE1: Sprite<Bgr565> = Sprite::new(Point::new(0, 0), &IMAGE);
//! const SPRITE2: Sprite<Bgr565> = Sprite::new(Point::new(32, 32), &IMAGE);
//! # let mut display = SimulatorDisplay::<Bgr565>::new(Size::new(128, 64));
//! SPRITE1.draw(&mut display).unwrap();
//! SPRITE2.draw(&mut display).unwrap();
//! ```

/// Store image data.
pub mod image;
/// given a [`Image`](crate::image::Image) a position and make it draw able
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
