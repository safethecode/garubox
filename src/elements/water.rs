use crate::types::Element;
use crate::grid::Grid;
use rand::Rng;

pub fn update(grid: &mut Grid, x: usize, y: usize, rng: &mut impl Rng) {
    if y + 1 >= grid.height() {
        return;
    }

    let below = grid.get(x, y + 1);

    if below == Element::Air {
        grid.swap_cells(x, y, x, y + 1);
        return;
    }

    let directions = if rng.gen_bool(0.5) {
        vec![-1, 1]
    } else {
        vec![1, -1]
    };

    for dx in directions.iter() {
        let nx = x as i32 + dx;
        if nx >= 0 && (nx as usize) < grid.width() {
            let below_side = grid.get(nx as usize, y + 1);
            if below_side == Element::Air {
                grid.swap_cells(x, y, nx as usize, y + 1);
                return;
            }
        }
    }

    for dx in directions {
        let nx = x as i32 + dx;
        if nx >= 0 && (nx as usize) < grid.width() {
            let side = grid.get(nx as usize, y);
            if side == Element::Air {
                grid.swap_cells(x, y, nx as usize, y);
                return;
            }
        }
    }
}
