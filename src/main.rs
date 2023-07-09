#![feature(int_roundings)]

use std::{fs::OpenOptions, io::BufWriter};

mod obj;
mod ppm;
mod raycast;
mod render;
use obj::load_obj;
use ppm::{Colour, Image};
use raycast::{Mesh, Plane, Vec3, Sphere};
use render::{Camera, PointLight, Scene};

const WIDTH: u16 = 500;
const HEIGHT: u16 = 500;

fn main() -> color_eyre::Result<()> {
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

    let mut bnuuy = load_obj(include_str!("../bnuuy.obj"));
    for triangle in &mut bnuuy {
        triangle.vertices = triangle.vertices.map(|pos| {
            pos + Vec3 {
                x: 1.0,
                y: 0.0,
                z: 6.0,
            }
        });
    }

    let mut scene = Scene {
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
                Box::new(Mesh::new(bnuuy)),
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

    for n in 0..48 {
        let mut image = Image::new(WIDTH, HEIGHT, Colour { r: 0, g: 0, b: 0 });

        let theta = n as f64 / 48.0 * std::f64::consts::TAU;

        scene.camera.pos.x = theta.sin() * 2.0;
        scene.camera.pos.z = theta.cos() * 2.0;

        scene.render(&mut image);

        let mut writer = BufWriter::with_capacity(
            19 + 12 * WIDTH as usize * HEIGHT as usize,
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(format!("animation/{n}.ppm"))?,
        );
        image.write_ppm(&mut writer)?;
    }

    Ok(())
}
