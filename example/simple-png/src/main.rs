use embedded_graphics::{
	geometry::Point,
	pixelcolor::Bgr565,
	prelude::{DrawTarget, RgbColor, Size},
	Drawable,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_sprites::{image::Image, include_image, sprite::Sprite};

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[include_image]
const IMG_GRASS: Image<Bgr565> = "img/grass.png";
#[include_image]
const IMG_FRAME: Image<Bgr565> = "img/frame.png";

const SPRITE_GRASS: Sprite<Bgr565> = Sprite::new(Point::new(15, 15), &IMG_GRASS);
const SPRITE_FRAME: Sprite<Bgr565> = Sprite::new(Point::new(70, 3), &IMG_FRAME);

fn main() {
	let mut display = SimulatorDisplay::<Bgr565>::new(Size::new(128, 64));
	display.clear(Bgr565::RED).unwrap();
	SPRITE_GRASS.draw(&mut display).unwrap();
	SPRITE_FRAME.draw(&mut display).unwrap();
	let output_settings = OutputSettingsBuilder::new().scale(10).build();
	Window::new(CARGO_PKG_NAME, &output_settings).show_static(&display);
}
