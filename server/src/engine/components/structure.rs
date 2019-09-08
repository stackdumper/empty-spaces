use super::super::types::Vector;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Debug, Serialize, Deserialize)]
pub struct Structure {
    pub sections: Vec<Vector>,
    pub center_of_mass: Vector,
}

impl Component for Structure {
    type Storage = VecStorage<Self>;
}

impl Structure {
    pub fn new() -> Self {
        Self {
            sections: vec![Vector::new(0.0, 0.0)],
            center_of_mass: Vector::new(0.0, 0.0),
        }
    }

    pub fn get_center_of_mass(pos_a: Vector, mass_a: f64, pos_b: Vector, mass_b: f64) -> Vector {
        let x = (mass_a * pos_a.x + mass_b * pos_b.x) / (mass_a + mass_b);
        let y = (mass_a * pos_a.y + mass_b * pos_b.y) / (mass_a + mass_b);

        Vector::new(x, y)
    }

    pub fn merge_structure(&mut self, self_position: &Vector, other_sections: Vec<Vector>) {
        // translate sections into local coordinate space
        // and append them to existing sections

        other_sections.into_iter().for_each(|mut v| {
            v.x = (v.x - self_position.x).round();
            v.y = (v.y - self_position.y).round();

            self.center_of_mass = Structure::get_center_of_mass(
                self.center_of_mass,
                self.sections.len() as f64,
                v,
                1.0,
            );

            self.sections.push(v)
        });
    }

    // return sections translated to global coordinate space
    pub fn get_sections(&self, self_position: &Vector) -> Vec<Vector> {
        self.sections
            .clone()
            .into_iter()
            .map(|mut v| {
                v.x += self_position.x;
                v.y += self_position.y;

                v
            })
            .collect()
    }

    pub fn get_mass(&self) -> f64 {
        self.sections.len() as f64
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_sections() {
        let structure = Structure::new();

        for section in structure.get_sections(&Vector::new(0.0, 0.0)) {
            assert_eq!(section.x, 0.0);
            assert_eq!(section.y, 0.0);
        }

        for section in structure.get_sections(&Vector::new(-2.0, 3.0)) {
            assert_eq!(section.x, -2.0);
            assert_eq!(section.y, 3.0);
        }
    }

    #[test]
    fn get_center_of_mass() {
        assert_eq!(
            Structure::get_center_of_mass(Vector::new(0.0, 0.0), 1.0, Vector::new(0.0, 1.0), 1.0),
            Vector::new(0.0, 0.5)
        )
    }

    #[test]
    fn test_merge_structure() {
        let mut a = Structure::new();
        // let b = Structure::new();

        a.merge_structure(&Vector::new(0.0, 0.0), vec![Vector::new(0.1, 1.1)]);

        assert_eq!(
            a.get_sections(&Vector::new(0.0, 0.0)),
            vec![Vector::new(0.0, 0.0), Vector::new(0.0, 1.0)]
        );

        assert_eq!(a.center_of_mass, Vector::new(0.0, 0.5))
    }
}
