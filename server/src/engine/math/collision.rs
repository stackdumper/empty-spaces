use super::super::types::Vector;
use cgmath::InnerSpace;

const SIZE: f64 = 1.0;
const HALF_SIZE: f64 = 0.5;

pub struct AABB {
    min: Vector,
    max: Vector,
}

impl AABB {
    pub fn new(position: &Vector) -> Self {
        Self {
            min: Vector::new(position.x - HALF_SIZE, position.y - HALF_SIZE),
            max: Vector::new(position.x + HALF_SIZE, position.y + HALF_SIZE),
        }
    }

    pub fn collides(a: &AABB, b: &AABB) -> bool {
        !(a.max.x < b.min.x || a.min.x > b.max.x || a.max.y < b.min.y || a.min.y > b.max.y)
    }

    pub fn new_complex(sections: &Vec<Vector>) -> AABB {
        let mut min = sections[0];
        let mut max = sections[0];

        for section in sections.iter() {
            if section.x < min.x {
                min.x = section.x;
            }

            if section.y < min.y {
                min.y = section.y;
            }

            if section.x > max.x {
                max.x = section.x;
            }

            if section.y > max.y {
                max.y = section.y;
            }
        }

        min.x -= HALF_SIZE;
        min.y -= HALF_SIZE;
        max.x += HALF_SIZE;
        max.y += HALF_SIZE;

        AABB { min, max }
    }

    pub fn collides_complex(sections_a: &Vec<Vector>, sections_b: &Vec<Vector>) -> bool {
        let mut aabbs_a = Vec::with_capacity(sections_a.len());
        for section in sections_a {
            aabbs_a.push(AABB::new(section));
        }

        let mut aabbs_b = Vec::with_capacity(sections_b.len());
        for section in sections_b {
            aabbs_b.push(AABB::new(section));
        }

        for a in aabbs_a.iter() {
            for b in aabbs_b.iter() {
                if AABB::collides(&a, &b) {
                    return true;
                }
            }
        }

        false
    }
}
