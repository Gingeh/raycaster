use std::{
    io::Write,
    iter,
    ops::{Add, Div, Mul},
};

#[derive(Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Add<Self> for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
        }
    }
}

impl Mul<Self> for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: (self.r as f64 * rhs.r as f64 / 255.0) as u8,
            g: (self.g as f64 * rhs.g as f64 / 255.0) as u8,
            b: (self.b as f64 * rhs.b as f64 / 255.0) as u8,
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: (self.r as f64 * rhs) as u8,
            g: (self.g as f64 * rhs) as u8,
            b: (self.b as f64 * rhs) as u8,
        }
    }
}

impl Div<f64> for Colour {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            r: (self.r as f64 / rhs) as u8,
            g: (self.g as f64 / rhs) as u8,
            b: (self.b as f64 / rhs) as u8,
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
