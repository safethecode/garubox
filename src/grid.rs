use crate::element::Element;
use crate::elements;
use ::rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PersonState {
    pub water_ticks: u32,
    pub jump_velocity: i32,
}

impl PersonState {
    pub fn new() -> Self {
        Self {
            water_ticks: 0,
            jump_velocity: 0,
        }
    }
}

pub struct Grid {
    cells: Vec<Vec<Element>>,
    width: usize,
    height: usize,
    person_states: HashMap<(usize, usize), PersonState>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            cells: vec![vec![Element::Air; width]; height],
            width,
            height,
            person_states: HashMap::new(),
        }
    }

    pub fn get_person_state(&self, x: usize, y: usize) -> Option<&PersonState> {
        self.person_states.get(&(x, y))
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Element {
        if x < self.width && y < self.height {
            self.cells[y][x]
        } else {
            Element::Stone
        }
    }

    pub fn set(&mut self, x: usize, y: usize, element: Element) {
        if x < self.width && y < self.height {
            let old_element = self.cells[y][x];
            self.cells[y][x] = element;

            if element == Element::Person && old_element != Element::Person {
                self.person_states.insert((x, y), PersonState::new());
            } else if element != Element::Person && old_element == Element::Person {
                self.person_states.remove(&(x, y));
            }
        }
    }

    pub fn swap_cells(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        if x1 < self.width && y1 < self.height && x2 < self.width && y2 < self.height {
            let temp = self.cells[y1][x1];
            self.cells[y1][x1] = self.cells[y2][x2];
            self.cells[y2][x2] = temp;

            let state1 = self.person_states.remove(&(x1, y1));
            let state2 = self.person_states.remove(&(x2, y2));
            if let Some(s) = state1 {
                self.person_states.insert((x2, y2), s);
            }
            if let Some(s) = state2 {
                self.person_states.insert((x1, y1), s);
            }
        }
    }

    pub fn set_person_state(&mut self, x: usize, y: usize, state: PersonState) {
        self.person_states.insert((x, y), state);
    }

    pub fn update(&mut self) {
        let mut rng = ::rand::thread_rng();

        for y in (0..self.height).rev() {
            let x_range: Vec<usize> = if rng.gen_bool(0.5) {
                (0..self.width).collect()
            } else {
                (0..self.width).rev().collect()
            };

            for x in x_range {
                let element = self.get(x, y);

                match element {
                    Element::Sand => elements::sand::update(self, x, y, &mut rng),
                    Element::Water => elements::water::update(self, x, y, &mut rng),
                    Element::Person => elements::person::update(self, x, y, &mut rng),
                    _ => {}
                }
            }
        }
    }

    pub fn place_element(&mut self, x: usize, y: usize, element: Element, radius: usize) {
        for dy in 0..=radius {
            for dx in 0..=radius {
                let dist_sq = dx * dx + dy * dy;
                if dist_sq <= radius * radius {
                    if x >= dx && y >= dy {
                        self.set(x - dx, y - dy, element);
                    }
                    if x + dx < self.width && y >= dy {
                        self.set(x + dx, y - dy, element);
                    }
                    if x >= dx && y + dy < self.height {
                        self.set(x - dx, y + dy, element);
                    }
                    if x + dx < self.width && y + dy < self.height {
                        self.set(x + dx, y + dy, element);
                    }
                }
            }
        }
    }
}
