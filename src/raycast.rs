use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalized(self) -> Self {
        self / self.length()
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    // :(
    pub fn cross(self, rhs: Self) -> Self {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;

        Self { x, y, z }
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

pub trait RayTarget {
    fn intersect(&self, origin: Vec3, direction: Vec3) -> Option<f64>;
    fn normal(&self, origin: Vec3, direction: Vec3) -> Vec3;
}

pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
}

impl RayTarget for Sphere {
    fn intersect(&self, origin: Vec3, direction: Vec3) -> Option<f64> {
        let direction = direction.normalized();
        let nabla = (direction.dot(origin - self.center)).powi(2)
            - (origin - self.center).length().powi(2)
            + self.radius.powi(2);

        if nabla < 0.0 {
            return None;
        }

        Some(-direction.dot(origin - self.center) - nabla.sqrt())
    }

    fn normal(&self, origin: Vec3, direction: Vec3) -> Vec3 {
        let pos = origin + direction * self.intersect(origin, direction).unwrap();
        (pos - self.center).normalized()
    }
}

pub struct Plane {
    pub normal: Vec3,
    pub pos: Vec3,
}

impl RayTarget for Plane {
    fn intersect(&self, origin: Vec3, direction: Vec3) -> Option<f64> {
        let dist =
            (self.pos.dot(self.normal) - origin.dot(self.normal)) / direction.dot(self.normal);

        Some(dist)
    }

    fn normal(&self, _: Vec3, _: Vec3) -> Vec3 {
        self.normal.normalized()
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
}

impl RayTarget for Triangle {
    fn intersect(&self, origin: Vec3, direction: Vec3) -> Option<f64> {
        let normal = self.normal(origin, direction);

        let possible_dist =
            (self.vertices[0].dot(normal) - origin.dot(normal)) / direction.dot(normal);

        let possible_intersection = origin + direction * possible_dist;

        let inside_ab = (self.vertices[1] - self.vertices[0])
            .cross(possible_intersection - self.vertices[0])
            .dot(normal)
            >= 0.0;
        let inside_bc = (self.vertices[2] - self.vertices[1])
            .cross(possible_intersection - self.vertices[1])
            .dot(normal)
            >= 0.0;
        let inside_ca = (self.vertices[0] - self.vertices[2])
            .cross(possible_intersection - self.vertices[2])
            .dot(normal)
            >= 0.0;

        if inside_ab && inside_bc && inside_ca {
            Some(possible_dist)
        } else {
            None
        }
    }

    fn normal(&self, _: Vec3, _: Vec3) -> Vec3 {
        (self.vertices[1] - self.vertices[0])
            .cross(self.vertices[2] - self.vertices[0])
            .normalized()
    }
}

impl RayTarget for Vec<Triangle> {
    fn intersect(&self, origin: Vec3, direction: Vec3) -> Option<f64> {
        self.iter()
            .filter_map(|tri| tri.intersect(origin, direction))
            .filter(|&dist| dist >= 0.0)
            .min_by(|a, b| a.partial_cmp(b).expect("No NaNs please"))
    }

    fn normal(&self, origin: Vec3, direction: Vec3) -> Vec3 {
        self.iter()
            .filter_map(|tri| tri.intersect(origin, direction).map(|dist| (tri, dist)))
            .filter(|&(_, dist)| dist >= 0.0)
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("No NaNs please"))
            .unwrap()
            .0
            .normal(origin, direction)
    }
}
