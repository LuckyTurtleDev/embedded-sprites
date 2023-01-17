extern crate proc_macro;

use embedded_graphics::pixelcolor::{RgbColor, *};
use image::{io::Reader as ImageReader, Pixel, RgbaImage};
use proc_macro::TokenStream;

#[proc_macro]
pub fn include_image(_item: TokenStream) -> TokenStream {
	type TargetColor = embedded_graphics::pixelcolor::Bgr565;
	let image = ImageReader::open("img/grass.png")
		.expect("failed to open image")
		.decode()
		.expect("failed to decode image");
	// convert input image  to vec of colors
	let image: RgbaImage = image.into_rgba8();
	let pixels = image.pixels();
	let mut colors: Vec<TargetColor> = Vec::new();
	for pixel in pixels {
		let mut chanel = pixel.channels().iter();
		// `Color::new()` does only cut bits; So create an Bgr888 and convert it to TargetColor
		let color = Bgr888::new(
			chanel.next().unwrap_or(&0).to_owned(),
			chanel.next().unwrap_or(&0).to_owned(),
			chanel.next().unwrap_or(&0).to_owned(),
		);
		let a = chanel.next().map(|value| value == &0).unwrap_or(false);
		colors.push(color.into());
	}
	// contruct output
	let tmap_array = "bitarr![const 0,0,]";
	let mut color_array = "[ ".to_owned();
	for color in colors.iter() {
		color_array += &format!("Color::new({}, {}, {}),", color.r(), color.g(), color.b());
	}
	color_array += "]";
	println!("{tmap_array}");
	let output = format!(
		"Image::new(&{},&{},{},{})",
		color_array,
		tmap_array,
		image.width(),
		image.height()
	);
	println!("{output}");
	output.parse().unwrap()
}
