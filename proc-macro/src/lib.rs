extern crate proc_macro;

use embedded_graphics::pixelcolor::*;
use image::{io::Reader as ImageReader, Pixel, RgbaImage};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

fn expand<C>(path: &str) -> TokenStream2
where
	C: RgbColor + From<Bgr888>,
{
	let image = ImageReader::open(path)
		.expect("failed to open image")
		.decode()
		.expect("failed to decode image");
	// convert input image  to vec of colors
	let image: RgbaImage = image.into_rgba8();
	let pixels = image.pixels();
	let mut colors: Vec<C> = Vec::new();
	for pixel in pixels {
		let mut chanel = pixel.channels().iter();
		// `Color::new()` only cuts bits; so we create a Bgr888 and convert it to
		// our target color type
		let color = Bgr888::new(
			chanel.next().unwrap_or(&0).to_owned(),
			chanel.next().unwrap_or(&0).to_owned(),
			chanel.next().unwrap_or(&0).to_owned(),
		);
		let a = chanel.next().map(|value| value == &0).unwrap_or(false);
		colors.push(color.into());
	}
	// contruct output
	let tmap_array = quote!(bitarr![const 0,0,]);
	let colors = colors.into_iter().map(|color| {
		let r = color.r();
		let g = color.g();
		let b = color.b();
		quote!(::embedded_graphics::pixelcolor::Bgr565::new(#r, #g, #b))
	});
	let color_array = quote!([#(#colors),*]);
	println!("{tmap_array}");
	let width = image.width() as u16;
	let height = image.height() as u16;
	quote! {
		Image::new(&#color_array, &#tmap_array, #width, #height)
	}
}

#[proc_macro]
pub fn include_image_bgr565(_item: TokenStream) -> TokenStream {
	expand::<Bgr565>("img/grass.png").into()
}
