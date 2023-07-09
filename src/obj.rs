use crate::raycast::{Triangle, Vec3};

pub fn load_obj(data: &str) -> Vec<Triangle> {
    let mut vertices = Vec::new();
    let mut face_indices: Vec<(usize, usize, usize)> = Vec::new();

    for line in data.lines() {
        match &line[0..2] {
            "v " => {
                let mut coords = line.split_ascii_whitespace().skip(1);
                vertices.push(Vec3 {
                    x: coords.next().unwrap().parse().unwrap(),
                    y: coords.next().unwrap().parse().unwrap(),
                    z: coords.next().unwrap().parse().unwrap(),
                });
            }
            "f " => {
                let mut indices = line.split_ascii_whitespace().skip(1);
                face_indices.push((
                    indices.next().unwrap().parse().unwrap(),
                    indices.next().unwrap().parse().unwrap(),
                    indices.next().unwrap().parse().unwrap(),
                ));
            }
            _ => {}
        }
    }

    let mut faces = Vec::new();
    for (a, b, c) in face_indices {
        faces.push(Triangle {
            vertices: [vertices[a - 1], vertices[b - 1], vertices[c - 1]],
        });
    }

    faces
}
