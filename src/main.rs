mod element;
mod grid;
mod ui;

use element::Element;
use grid::Grid;
use ui::GameUI;

const GRID_WIDTH: usize = 200;
const GRID_HEIGHT: usize = 150;
const CELL_SIZE: f32 = 4.0;

#[macroquad::main("Garubox - Falling Sand Game")]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut ui = GameUI::new(CELL_SIZE);

    for x in 0..GRID_WIDTH {
        grid.set(x, 0, Element::Stone);
        grid.set(x, GRID_HEIGHT - 1, Element::Stone);
    }
    for y in 0..GRID_HEIGHT {
        grid.set(0, y, Element::Stone);
        grid.set(GRID_WIDTH - 1, y, Element::Stone);
    }

    loop {
        ui.handle_input(&mut grid);

        if !ui.paused {
            grid.update();
        }

        ui.render(&grid);

        macroquad::prelude::next_frame().await
    }
}
