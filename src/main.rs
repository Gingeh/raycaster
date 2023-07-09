#![feature(int_roundings)]

use std::{fs::OpenOptions, io::BufWriter};

mod obj;
mod ppm;
mod raycast;
mod render;
use obj::load_obj;
use ppm::{Colour, Image};
use raycast::{Mesh, Plane, Triangle, Vec3};
use render::{Camera, PointLight, Scene};

const WIDTH: u16 = 500;
const HEIGHT: u16 = 500;

fn main() -> color_eyre::Result<()> {
    let rat = load_obj(include_str!("../rat.obj"));

    let wall = Plane {
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        pos: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 2.0,
        },
    };

    let message_width = 3.2;
    let message_height = 1.0;
    let y_offset = 2.0;
    let message = Mesh::new(vec![
        Triangle {
            vertices: [
                Vec3 {
                    x: -message_width / 2.0,
                    y: y_offset - message_height / 2.0,
                    z: -2.01,
                },
                Vec3 {
                    x: -message_width / 2.0,
                    y: y_offset + message_height / 2.0,
                    z: -2.0,
                },
                Vec3 {
                    x: message_width / 2.0,
                    y: y_offset - message_height / 2.0,
                    z: -2.01,
                },
            ],
        },
        Triangle {
            vertices: [
                Vec3 {
                    x: message_width / 2.0,
                    y: y_offset - message_height / 2.0,
                    z: -2.01,
                },
                Vec3 {
                    x: -message_width / 2.0,
                    y: y_offset + message_height / 2.0,
                    z: -2.0,
                },
                Vec3 {
                    x: message_width / 2.0,
                    y: y_offset + message_height / 2.0,
                    z: -2.0,
                },
            ],
        },
    ]);

    let mut scene = Scene {
        objects: vec![
            (
                Box::new(Mesh::new(rat.clone())),
                Colour {
                    r: 255,
                    g: 0,
                    b: 63,
                },
            ),
            (
                Box::new(wall),
                Colour {
                    r: 85,
                    g: 205,
                    b: 252,
                },
            ),
            (Box::new(message), Colour { r: 0, g: 255, b: 0 }),
        ],
        camera: Camera::new(
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: -10.0,
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
                y: 3.0,
                z: -5.0,
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
        let mut rotated_rat = rat.clone();
        for vertex in rotated_rat.iter_mut().flat_map(|tri| &mut tri.vertices) {
            let new_x = vertex.x * theta.cos() - vertex.z * theta.sin();
            let new_z = vertex.z * theta.cos() + vertex.x * theta.sin();
            *vertex = Vec3 {
                x: new_x,
                y: vertex.y,
                z: new_z,
            }
        }

        scene.objects[0].0 = Box::new(Mesh::new(rotated_rat));

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
