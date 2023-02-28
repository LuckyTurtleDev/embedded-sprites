# embedded-sprites ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![embedded-sprites on crates.io](https://img.shields.io/crates/v/embedded-sprites)](https://crates.io/crates/embedded-sprites) [![embedded-sprites on docs.rs](https://docs.rs/embedded-sprites/badge.svg)](https://docs.rs/embedded-sprites) [![Source Code Repository](https://img.shields.io/badge/Code-On%20none-blue)](none) ![Rust Version: none](https://img.shields.io/badge/rustc--orange.svg)

Embedded no std graphics library for bundling image at compile time, to be used at the [embedded-graphics][__link0] crate.

The [`include_image`][__link1] macro can be usede to create a [`Image`][__link2] from an existing image file at compile time. Every image formats supported by the [image crate][__link3] can be used. The image will be automatically be converted to the requested pixelcolor. Current only rgb pixelcolors are supported.


```rust
use embedded_graphics::pixelcolor::Bgr565;
use embedded_sprites::{image::Image, include_image};
#[include_image]
const IMAGE: Image<Bgr565> = "grass.png";
```

To draw a [`Image`][__link4] it must be put inside a [`Sprite`][__link5]. You can use the same [`Image`][__link6] inside multiple [`Sprite`][__link7]s;


```rust
use embedded_graphics::{geometry::Point, pixelcolor::Bgr565, Drawable};
use embedded_sprites::sprite::Sprite;

const SPRITE1: Sprite<Bgr565> = Sprite::new(Point::new(0, 0), &IMAGE);
const SPRITE2: Sprite<Bgr565> = Sprite::new(Point::new(32, 32), &IMAGE);
SPRITE1.draw(&mut display).unwrap();
SPRITE2.draw(&mut display).unwrap();
```


 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEGyDwipHVMb5RGxgd3zutc1TvG3ARKV4UcQ1NGyM1aXabIPYbYXKEG3UKlWy8vTi0G4kv9VENJTIRG2q_VeeZzLcpG8a3zQR9jj8BYWSBg3BlbWJlZGRlZC1zcHJpdGVzZTAuMS4xcGVtYmVkZGVkX3Nwcml0ZXM
 [__link0]: https://crates.io/crates/embedded-graphics
 [__link1]: https://docs.rs/embedded-sprites/0.1.1/embedded_sprites/?search=include_image
 [__link2]: https://docs.rs/embedded-sprites/0.1.1/embedded_sprites/?search=image::Image
 [__link3]: https://crates.io/crates/image
 [__link4]: https://docs.rs/embedded-sprites/0.1.1/embedded_sprites/?search=image::Image
 [__link5]: https://docs.rs/embedded-sprites/0.1.1/embedded_sprites/?search=sprite::Sprite
 [__link6]: https://docs.rs/embedded-sprites/0.1.1/embedded_sprites/?search=image::Image
 [__link7]: https://docs.rs/embedded-sprites/0.1.1/embedded_sprites/?search=sprite::Sprite
