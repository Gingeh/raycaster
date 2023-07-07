use std::cmp;

use crate::{
    ppm::{Colour, Image},
    raycast::{RayTarget, Vec3},
};

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
        let right = forward.cross(up).normalized();

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
    pub objects: Vec<(Box<dyn RayTarget>, Colour)>,
    pub camera: Camera,
    pub lights: Vec<PointLight>,
}

impl Scene {
    pub fn render(self, image: &mut Image) {
        let (width, height) = (image.width(), image.height());

        for (x, y, colour) in image.iter_mut() {
            let right = self.camera.right
                * ((x as f64 - width as f64 / 2.0) / cmp::min(width, height) as f64);
            let up = self.camera.up
                * ((y as f64 - height as f64 / 2.0) / cmp::min(width, height) as f64);
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
                continue;
            };

            let intersection = ray * dist + self.camera.pos;
            let normal = target.normal(intersection);

            let intensity: f64 = self
                .lights
                .iter()
                .map(|light| {
                    (light.pos - intersection)
                        .normalized()
                        .dot(normal)
                        .clamp(0.0, f64::INFINITY)
                })
                .sum();

            *colour = new_colour * intensity;
        }
    }
}
