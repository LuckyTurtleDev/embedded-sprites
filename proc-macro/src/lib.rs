use embedded_graphics::pixelcolor::{Bgr888, RgbColor as _};
use image::{io::Reader as ImageReader, Pixel, RgbaImage};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::path::PathBuf;
use syn::{parse_macro_input, spanned::Spanned as _, Expr, ExprLit, ItemConst, Lit};

fn expand(
	ItemConst {
		attrs,
		vis,
		const_token,
		ident,
		colon_token,
		ty,
		eq_token,
		expr,
		semi_token,
	}: ItemConst,
) -> syn::Result<TokenStream2> {
	let path_lit = match expr.as_ref() {
		Expr::Lit(ExprLit { lit: Lit::Str(path), .. }) => path,
		expr => return Err(syn::Error::new(expr.span(), "Expected path to image")),
	};
	let path: PathBuf = path_lit
		.value()
		.parse()
		.map_err(|err| syn::Error::new(path_lit.span(), format!("Invalid path: {err}")))?;
	let image = ImageReader::open(&path)
		.map_err(|err| syn::Error::new(path_lit.span(), format!("Failed to open image: {err}")))?
		.decode()
		.map_err(|err| syn::Error::new(path_lit.span(), format!("Failed to decode image: {err}")))?;
	let path = path
		.canonicalize()
		.ok()
		.and_then(|path| path.to_str().map(String::from))
		.unwrap_or_else(|| path_lit.value());

	// convert input image to vec of colors
	let image: RgbaImage = image.into_rgba8();
	let pixels = image.pixels();
	let mut colors: Vec<Bgr888> = Vec::new();
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
		colors.push(color);
	}

	// contruct output
	let tmap_array = quote!(bitarr![const 0,0,]);
	let color_ty = quote!(<#ty as ::embedded_sprites::private::Image>::Color);
	let colors = colors.into_iter().map(|color| {
		let r = color.r();
		let g = color.g();
		let b = color.b();
		quote!({
			let (r, g, b) = ::embedded_sprites::private::convert_from_bgr888::
				<#color_ty>(#r, #g, #b);
			#color_ty::new(r, g, b)
		})
	});
	let color_array = quote!([#(#colors),*]);
	let width = image.width() as u16;
	let height = image.height() as u16;
	let output = quote! {
		#(#attrs)* #vis #const_token #ident #colon_token #ty #eq_token {
			// include the bytes so that the compiler knows to recompile when the
			// image file changes
			const _: &[u8] = ::core::include_bytes!(#path);

			const COLOR_ARRAY: &[#color_ty] = &#color_array;
			match ::embedded_sprites::image::Image::<'static, #color_ty>::new(
				&COLOR_ARRAY, &#tmap_array, #width, #height
			) {
				::core::result::Result::Ok(img) => img,
				_ => panic!("Failed to construct image")
			}
		}
		#semi_token
	};
	//eprintln!("{output}");
	Ok(output)
}

#[proc_macro_attribute]
pub fn include_image(_attr: TokenStream, item: TokenStream) -> TokenStream {
	expand(parse_macro_input!(item))
		.unwrap_or_else(|err| err.into_compile_error())
		.into()
}
