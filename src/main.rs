#![feature(int_roundings)]

use std::io;

mod ppm;
mod raycast;
mod render;
use ppm::{Colour, Image};
use raycast::{Plane, Sphere, Vec3};
use render::{Camera, PointLight, Scene};

const WIDTH: u16 = 400;
const HEIGHT: u16 = 300;

fn main() -> color_eyre::Result<()> {
    let mut image = Image::new(WIDTH, HEIGHT, Colour { r: 0, g: 0, b: 0 });
    let sphere1 = Sphere {
        radius: 5.0,
        center: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 15.0,
        },
    };

    let sphere2 = Sphere {
        radius: 2.0,
        center: Vec3 {
            x: -1.0,
            y: 0.0,
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
                x: 0.0,
                y: 10.0,
                z: 6.0,
            },
            colour: Colour {
                r: 255,
                g: 255,
                b: 255,
            },
        }],
    };
    scene.render(&mut image);
    image.write_ppm(&mut io::stdout())?;
    Ok(())
}
