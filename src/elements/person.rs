use crate::element::Element;
use crate::grid::{Grid, PersonState};
use rand::Rng;

pub fn update(grid: &mut Grid, x: usize, y: usize, rng: &mut impl Rng) {
    if y + 1 >= grid.height() {
        return;
    }

    let mut state = grid.get_person_state(x, y).cloned().unwrap_or(PersonState::new());

    let below = grid.get(x, y + 1);
    let above = if y > 0 { grid.get(x, y - 1) } else { Element::Air };

    let is_suffocating = matches!(above, Element::Water | Element::Sand);

    if is_suffocating {
        state.water_ticks += 1;
        if state.water_ticks >= 600 {
            grid.set(x, y, Element::Air);
            return;
        }
    } else {
        state.water_ticks = 0;
    }

    if state.jump_velocity > 0 {
        if y > 0 {
            let above = grid.get(x, y - 1);
            if above == Element::Air || above == Element::Water {
                grid.swap_cells(x, y, x, y - 1);
                state.jump_velocity -= 1;
                grid.set_person_state(x, y - 1, state);
                return;
            } else {
                state.jump_velocity = 0;
            }
        } else {
            state.jump_velocity = 0;
        }
    }

    if below == Element::Air || below == Element::Water {
        grid.swap_cells(x, y, x, y + 1);
        grid.set_person_state(x, y + 1, state);
        return;
    }

    if below.can_walk_on() || below == Element::Person {
        let directions = if rng.gen_bool(0.5) {
            vec![-1, 1]
        } else {
            vec![1, -1]
        };

        for dx in directions {
            let nx = x as i32 + dx;
            if nx < 0 || nx as usize >= grid.width() {
                continue;
            }
            let nx = nx as usize;

            let target = grid.get(nx, y);
            let target_below = if y + 1 < grid.height() { grid.get(nx, y + 1) } else { Element::Stone };

            if target == Element::Air && (target_below.can_walk_on() || target_below == Element::Person) {
                grid.swap_cells(x, y, nx, y);
                grid.set_person_state(nx, y, state);
                return;
            }

            if y > 0 && target.can_walk_on() {
                let above_target = grid.get(nx, y - 1);
                if above_target == Element::Air {
                    grid.swap_cells(x, y, nx, y - 1);
                    grid.set_person_state(nx, y - 1, state);
                    return;
                }
            }

            if target != Element::Air && y > 0 && rng.gen_bool(0.3) {
                let above = grid.get(x, y - 1);
                if above == Element::Air || above == Element::Water {
                    state.jump_velocity = 2;
                    grid.set_person_state(x, y, state.clone());
                    return;
                }
            }
        }
    }

    grid.set_person_state(x, y, state);
}
