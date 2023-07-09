use std::{cmp, ops::Add, sync::Mutex, thread};

use crate::{
    ppm::{Colour, Image},
    raycast::{RayTarget, Vec3},
};

const SHADOW_BIAS: f64 = 0.000_000_000_001;
const AMBIENT_COEF: f64 = 0.1;

// yucky
pub struct Camera {
    pos: Vec3,
    forward: Vec3,
    up: Vec3,
    right: Vec3,
    focal_dist: f64,
}

impl Camera {
    pub fn new(pos: Vec3, forward: Vec3, up: Vec3, focal_dist: f64) -> Self {
        let right = up.cross(forward).normalized();

        Self {
            pos,
            forward: forward.normalized(),
            up: up.normalized(),
            right,
            focal_dist,
        }
    }
}

pub struct PointLight {
    pub pos: Vec3,
    pub colour: Colour,
}

pub struct Scene {
    pub objects: Vec<(Box<dyn RayTarget + Sync>, Colour)>,
    pub camera: Camera,
    pub lights: Vec<PointLight>,
}

impl Scene {
    pub fn render(self, image: &mut Image) {
        let (width, height) = (image.width(), image.height());

        let mut pixels = image.iter_mut().collect::<Vec<_>>();
        let chunks = Mutex::new(pixels.chunks_mut(width as usize));

        thread::scope(|scope| {
            for _ in 0..8 {
                scope.spawn(|| loop {
                    match {
                        let mut chunks_guard = chunks.lock().unwrap();
                        chunks_guard.next()
                    } {
                        None => return,
                        Some(chunk) => {
                            for (x, y, colour) in chunk {
                                self.render_pixel(width, height, *x, *y, colour);
                            }
                        }
                    }
                });
            }
        });
    }

    fn render_pixel(&self, width: u16, height: u16, x: u16, y: u16, colour: &mut Colour) {
        let right =
            self.camera.right * ((x as f64 - width as f64 / 2.0) / cmp::min(width, height) as f64);
        let up =
            self.camera.up * ((y as f64 - height as f64 / 2.0) / cmp::min(width, height) as f64);
        let forward = self.camera.forward * self.camera.focal_dist;

        let ray = (right + up + forward).normalized();

        let Some((target, dist, &new_colour)) = self
            .objects
            .iter()
            .filter_map(|(target, colour)| {
                target
                    .intersect(self.camera.pos, ray)
                    .map(|dist| (target, dist, colour))
            })
            .filter(|&(_, dist, _)| dist >= 0.0)
            .min_by(|(_, a, _), (_, b, _)| a.partial_cmp(b).expect("No NaNs please"))
        else {
            return;
        };

        let normal = target.normal(self.camera.pos, ray);
        let intersection = ray * dist + self.camera.pos;

        let diffuse_light: Colour = self
            .lights
            .iter()
            .filter(|light| {
                self.objects
                    .iter()
                    .filter_map(|(target, _)| {
                        target.intersect(light.pos, (intersection - light.pos).normalized())
                    })
                    .all(|dist| dist + SHADOW_BIAS >= (intersection - light.pos).length())
            })
            .map(|light| {
                light.colour
                    * (light.pos - intersection)
                        .normalized()
                        .dot(normal)
                        .clamp(0.0, f64::INFINITY)
            })
            .fold(Colour { r: 0, g: 0, b: 0 }, Add::add);

        let ambient_light = self
            .lights
            .iter()
            .map(|light| light.colour)
            .fold(Colour { r: 0, g: 0, b: 0 }, Add::add)
            / self.lights.len() as f64
            * AMBIENT_COEF;

        *colour = new_colour * (diffuse_light + ambient_light);
    }
}
