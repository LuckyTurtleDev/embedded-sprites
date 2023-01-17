use bitvec::{bitarr, prelude::*};
use embedded_graphics::{geometry::Point, pixelcolor::Bgr565, prelude::Size, Drawable, Pixel};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_sprites::{image::Image, include_image, sprite::Sprite};
use konst::result::unwrap_ctx;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[include_image]
const IMAGE: Image<Bgr565> = "img/grass.png";

const SPRITE: Sprite<Bgr565> = Sprite::new(Point::new(15, 15), &IMAGE);

fn main() {
	let mut display = SimulatorDisplay::<Bgr565>::new(Size::new(128, 64));
	Pixel(Point::new(0, 10), Bgr565::new(0, 255, 0)).draw(&mut display).unwrap();
	SPRITE.draw(&mut display).unwrap();
	let output_settings = OutputSettingsBuilder::new().scale(10).build();
	Window::new(CARGO_PKG_NAME, &output_settings).show_static(&display);
}
