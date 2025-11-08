use crate::element::Element;
use ::rand::Rng;

pub struct Grid {
    cells: Vec<Vec<Element>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            cells: vec![vec![Element::Air; width]; height],
            width,
            height,
        }
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
            self.cells[y][x] = element;
        }
    }

    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        if x1 < self.width && y1 < self.height && x2 < self.width && y2 < self.height {
            let temp = self.cells[y1][x1];
            self.cells[y1][x1] = self.cells[y2][x2];
            self.cells[y2][x2] = temp;
        }
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
                    Element::Sand => self.update_sand(x, y, &mut rng),
                    Element::Water => self.update_water(x, y, &mut rng),
                    _ => {}
                }
            }
        }
    }

    fn update_sand(&mut self, x: usize, y: usize, rng: &mut impl Rng) {
        if y + 1 >= self.height {
            return;
        }

        let below = self.get(x, y + 1);

        if below == Element::Air || below == Element::Water {
            self.swap(x, y, x, y + 1);
            return;
        }

        let directions = if rng.gen_bool(0.5) {
            vec![-1, 1]
        } else {
            vec![1, -1]
        };

        for dx in directions {
            let nx = x as i32 + dx;
            if nx >= 0 && (nx as usize) < self.width {
                let below_side = self.get(nx as usize, y + 1);
                if below_side == Element::Air || below_side == Element::Water {
                    self.swap(x, y, nx as usize, y + 1);
                    return;
                }
            }
        }
    }

    fn update_water(&mut self, x: usize, y: usize, rng: &mut impl Rng) {
        if y + 1 >= self.height {
            return;
        }

        let below = self.get(x, y + 1);

        if below == Element::Air {
            self.swap(x, y, x, y + 1);
            return;
        }

        let directions = if rng.gen_bool(0.5) {
            vec![-1, 1]
        } else {
            vec![1, -1]
        };

        for dx in directions.iter() {
            let nx = x as i32 + dx;
            if nx >= 0 && (nx as usize) < self.width {
                let below_side = self.get(nx as usize, y + 1);
                if below_side == Element::Air {
                    self.swap(x, y, nx as usize, y + 1);
                    return;
                }
            }
        }

        for dx in directions {
            let nx = x as i32 + dx;
            if nx >= 0 && (nx as usize) < self.width {
                let side = self.get(nx as usize, y);
                if side == Element::Air {
                    self.swap(x, y, nx as usize, y);
                    return;
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
