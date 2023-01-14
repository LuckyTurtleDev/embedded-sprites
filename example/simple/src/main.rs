use bitvec::{bitarr, prelude::*};
use embedded_graphics::{geometry::Point, pixelcolor::Bgr565, prelude::Size, Drawable, Pixel};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_sprites::{image::Image, sprite::Sprite};
use konst::result::unwrap_ctx;

type Color = Bgr565;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

const IMAGE: Image<Color> = unwrap_ctx!(Image::new(
	&[
		Color::new(255, 0, 0),
		Color::new(0, 255, 0),
		Color::new(0, 0, 255),
		Color::new(255, 0, 255),
		Color::new(255, 255, 255),
		Color::new(255, 255, 255),
	],
	&bitarr![const 0,0,0,0,1,0],
	3,
	2
));

const SPRITE: Sprite<Color> = Sprite::new(Point::new(15, 15), &IMAGE);

fn main() {
	let mut display = SimulatorDisplay::<Bgr565>::new(Size::new(128, 64));
	Pixel(Point::new(0, 10), Bgr565::new(0, 255, 0)).draw(&mut display).unwrap();
	SPRITE.draw(&mut display).unwrap();
	let output_settings = OutputSettingsBuilder::new().scale(10).build();
	Window::new(CARGO_PKG_NAME, &output_settings).show_static(&display);
}
