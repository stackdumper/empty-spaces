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
        for a_section in sections_a {
            let a = AABB::new(&a_section);
            for b_section in sections_b {
                // if (b_section - a_section).magnitude() <= 1.0 {
                let b = AABB::new(&b_section);

                if AABB::collides(&a, &b) {
                    return true;
                }
                // }
            }
        }
        false
    }
}

// #[derive(Debug)]
// pub struct AABB {
//     position: Vector,
//     min: Vector,
//     max: Vector,
// }

// pub fn get_aabb_single(p: &Vector) -> AABB {
//     let aabb = AABB {
//         position: p.clone(),
//         min: Vector::new(p.x - SIZE / 2.0, p.y - SIZE / 2.0),
//         max: Vector::new(p.x + SIZE / 2.0, p.y + SIZE / 2.0),
//     };

//     aabb
// }

// pub fn get_aabb(p: &Vector, sections: &Vec<Vector>) -> AABB {
//     let min = sections.into_iter().fold(sections[0], |a, b| {
//         Vector::new(
//             if a.x < b.x { a.x } else { b.x },
//             if a.y < b.y { a.y } else { b.y },
//         )
//     });

//     let max = sections.into_iter().fold(sections[0], |a, b| {
//         Vector::new(
//             if a.x > b.x { a.x } else { b.x },
//             if a.y > b.y { a.y } else { b.y },
//         )
//     });

//     let aabb = AABB {
//         position: p.clone(),
//         min: Vector::new(min.x - SIZE / 2.0, min.y - SIZE / 2.0),
//         max: Vector::new(max.x + SIZE / 2.0, max.y + SIZE / 2.0),
//     };

//     aabb
// }

// pub fn might_collide(a: &AABB, b: &AABB) -> bool {
//     if a.max.x < b.min.x || a.min.x > b.max.x {
//         return false;
//     }
//     if a.max.y < b.min.y || a.min.y > b.max.y {
//         return false;
//     }

//     return true;
// }

// pub fn has_collision(a_sections: &Vec<Vector>, b_sections: &Vec<Vector>) -> bool {
//     for a_section in a_sections {
//         let a = get_aabb_single(&a_section);
//         for b_section in b_sections {
//             if (b_section - a_section).magnitude() <= 1.0 {
//                 let b = get_aabb_single(&b_section);

//                 if might_collide(&a, &b) {
//                     return true;
//                 }
//             }
//         }
//     }
//     false
// }

// pub fn get_collision_normal(a: &AABB, b: &AABB) -> Vector {
//     let normal = b.position - a.position;

//     // Calculate half extents along x axis for each object
//     let a_extent = (a.max.x - a.min.x) / 2.0;
//     let b_extent = (b.max.x - b.min.x) / 2.0;

//     let x_overlap = a_extent + b_extent - normal.x.abs();

//     if x_overlap > 0.0 {
//         let a_extent = (a.max.y - a.min.y) / 2.0;
//         let b_extent = (b.max.y - b.min.y) / 2.0;

//         let y_overlap = a_extent + b_extent - normal.y.abs();

//         if y_overlap > 0.0 {
//             if x_overlap < y_overlap {
//                 if normal.x < 0.0 {
//                     return Vector::new(-1.0, 0.0);
//                 } else {
//                     return Vector::new(0.0, 0.0);
//                 }
//             } else {
//                 if normal.y < 0.0 {
//                     return Vector::new(0.0, -1.0);
//                 } else {
//                     return Vector::new(0.0, 1.0);
//                 }
//             }
//         }
//     }

//     return Vector::new(0.0, 0.0);
// }
