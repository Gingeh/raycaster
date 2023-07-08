#![feature(int_roundings)]

use std::{fs::OpenOptions, io::BufWriter};

mod ppm;
mod raycast;
mod render;
use ppm::{Colour, Image};
use raycast::{Plane, Sphere, Triangle, Vec3};
use render::{Camera, PointLight, Scene};

const WIDTH: u16 = 400;
const HEIGHT: u16 = 300;

fn main() -> color_eyre::Result<()> {
    let mut image = Image::new(WIDTH, HEIGHT, Colour { r: 0, g: 0, b: 0 });
    let sphere1 = Sphere {
        radius: 5.0,
        center: Vec3 {
            x: 0.0,
            y: 2.0,
            z: 15.0,
        },
    };

    let sphere2 = Sphere {
        radius: 1.0,
        center: Vec3 {
            x: -2.0,
            y: 0.5,
            z: 7.0,
        },
    };

    let plane = Plane {
        normal: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        pos: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };

    let cube = make_cube(
        Vec3 {
            x: 1.0,
            y: 0.75,
            z: 7.5,
        },
        0.75,
    );

    let scene = Scene {
        objects: vec![
            (
                Box::new(sphere1),
                Colour {
                    r: 85,
                    g: 205,
                    b: 252,
                },
            ),
            (
                Box::new(sphere2),
                Colour {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            ),
            (
                Box::new(plane),
                Colour {
                    r: 247,
                    g: 168,
                    b: 184,
                },
            ),
            (
                Box::new(cube),
                Colour {
                    r: 255,
                    g: 0,
                    b: 63,
                },
            ),
        ],
        camera: Camera::new(
            Vec3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            1.0,
        ),
        lights: vec![PointLight {
            pos: Vec3 {
                x: -2.0,
                y: 7.0,
                z: 4.0,
            },
            colour: Colour {
                r: 255,
                g: 255,
                b: 255,
            },
        }],
    };
    scene.render(&mut image);

    let mut writer = BufWriter::with_capacity(
        19 + 12 * WIDTH as usize * HEIGHT as usize,
        OpenOptions::new()
            .create(true)
            .write(true)
            .open("out.ppm")?,
    );
    image.write_ppm(&mut writer)?;

    Ok(())
}

fn make_cube(center: Vec3, radius: f64) -> Vec<Triangle> {
    let vertices = [
        Vec3 {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        },
        Vec3 {
            x: 1.0,
            y: -1.0,
            z: -1.0,
        },
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: -1.0,
        },
        Vec3 {
            x: -1.0,
            y: 1.0,
            z: -1.0,
        },
        Vec3 {
            x: 1.0,
            y: -1.0,
            z: 1.0,
        },
        Vec3 {
            x: -1.0,
            y: -1.0,
            z: 1.0,
        },
        Vec3 {
            x: -1.0,
            y: 1.0,
            z: 1.0,
        },
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    ]
    .map(|pos| pos * radius + center);

    let make_tri = |a: usize, b: usize, c: usize| -> Triangle {
        Triangle {
            vertices: [vertices[a], vertices[b], vertices[c]],
        }
    };

    vec![
        make_tri(0, 1, 5),
        make_tri(5, 1, 4),
        make_tri(1, 2, 4),
        make_tri(4, 2, 7),
        make_tri(2, 3, 7),
        make_tri(7, 3, 6),
        make_tri(3, 0, 6),
        make_tri(6, 0, 5),
        make_tri(6, 5, 7),
        make_tri(7, 5, 4),
        make_tri(2, 1, 3),
        make_tri(3, 1, 0),
    ]
}
