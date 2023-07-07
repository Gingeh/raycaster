use std::{io::Write, iter, ops::Mul};

#[derive(Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: (self.r as f64 * rhs).clamp(0.0, 256.0 - f64::EPSILON) as u8,
            g: (self.g as f64 * rhs).clamp(0.0, 256.0 - f64::EPSILON) as u8,
            b: (self.b as f64 * rhs).clamp(0.0, 256.0 - f64::EPSILON) as u8,
        }
    }
}

pub struct Image {
    width: u16,
    height: u16,
    buffer: Box<[Colour]>,
}

impl Image {
    pub fn new(width: u16, height: u16, fill: Colour) -> Self {
        Self {
            width,
            height,
            buffer: iter::repeat(fill)
                .take(width as usize * height as usize)
                .collect(),
        }
    }

    pub fn write_ppm(&self, file: &mut impl Write) -> color_eyre::Result<()> {
        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;

        for colour in &*self.buffer {
            writeln!(file, "{} {} {}", colour.r, colour.g, colour.b)?;
        }

        Ok(())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u16, u16, &mut Colour)> {
        self.buffer.iter_mut().enumerate().map(|(idx, colour)| {
            (
                idx.rem_euclid(self.width as usize) as u16,
                self.height - idx.div_floor(self.width as usize) as u16,
                colour,
            )
        })
    }

    pub const fn width(&self) -> u16 {
        self.width
    }

    pub const fn height(&self) -> u16 {
        self.height
    }
}
