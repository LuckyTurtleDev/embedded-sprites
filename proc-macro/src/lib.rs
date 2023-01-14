extern crate proc_macro;

use image::{io::Reader as ImageReader, ImageOutputFormat, Pixel, RgbaImage};
use proc_macro::TokenStream;

#[derive(Debug)]
struct Color {
	r: u8,
	g: u8,
	b: u8,
	a: bool,
}

#[proc_macro]
pub fn include_image(_item: TokenStream) -> TokenStream {
	let image = ImageReader::open("img/grass.png")
		.expect("failed to open image")
		.decode()
		.expect("failed to decode image");
	let image: RgbaImage = image.into_rgba8();
	let pixels = image.pixels();
	let mut colors = Vec::new();
	for pixel in pixels {
		let mut chanel = pixel.channels().iter();
		let color = Color {
			r: chanel.next().unwrap_or(&0).to_owned(),
			g: chanel.next().unwrap_or(&0).to_owned(),
			b: chanel.next().unwrap_or(&0).to_owned(),
			a: chanel.next().map(|value| value == &0).unwrap_or(false),
		};
		colors.push(color)
	}
	let output = format!("{colors:#?}");
	let output = "r#\"".to_owned() + &output + "\"#";
	output.parse().unwrap()
}
