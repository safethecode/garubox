use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Element {
    Air,
    Sand,
    Water,
    Stone,
}

impl Element {
    pub fn color(&self) -> Color {
        match self {
            Element::Air => BLACK,
            Element::Sand => Color::new(0.76, 0.70, 0.50, 1.0),
            Element::Water => Color::new(0.2, 0.4, 0.8, 1.0),
            Element::Stone => Color::new(0.5, 0.5, 0.5, 1.0),
        }
    }

    #[allow(dead_code)]
    pub fn is_solid(&self) -> bool {
        !matches!(self, Element::Air | Element::Water)
    }

    #[allow(dead_code)]
    pub fn is_liquid(&self) -> bool {
        matches!(self, Element::Water)
    }

    #[allow(dead_code)]
    pub fn is_movable(&self) -> bool {
        matches!(self, Element::Sand | Element::Water)
    }
}
