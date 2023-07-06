#![feature(int_roundings)]

use std::{cmp, io};

mod ppm;
use ppm::{Colour, Image};

const WIDTH: u16 = 1920;
const HEIGHT: u16 = 1080;

fn main() -> color_eyre::Result<()> {
    let mut image = Image::new(WIDTH, HEIGHT, Colour { r: 0, g: 0, b: 0 });

    for (x, y, colour) in image.iter_mut() {
        *colour = Colour {
            r: (x as f32 / WIDTH as f32 * 255.0) as u8,
            g: (y as f32 / HEIGHT as f32 * 255.0) as u8,
            b: if ((x as f32 - (WIDTH as f32 / 2.0)).powi(2)
                + (y as f32 - (HEIGHT as f32 / 2.0)).powi(2))
                <= (cmp::min(WIDTH, HEIGHT) as f32 / 2.0).powi(2)
            {
                255
            } else {
                0
            },
        }
    }

    image.write_ppm(&mut io::stdout())?;
    Ok(())
}
