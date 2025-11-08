use crate::element::Element;
use crate::grid::Grid;
use macroquad::prelude::*;

pub struct GameUI {
    pub selected_element: Element,
    pub paused: bool,
    cell_size: f32,
}

impl GameUI {
    pub fn new(cell_size: f32) -> Self {
        Self {
            selected_element: Element::Sand,
            paused: false,
            cell_size,
        }
    }

    pub fn handle_input(&mut self, grid: &mut Grid) {
        if is_key_pressed(KeyCode::Key1) {
            self.selected_element = Element::Sand;
        }
        if is_key_pressed(KeyCode::Key2) {
            self.selected_element = Element::Water;
        }
        if is_key_pressed(KeyCode::Key3) {
            self.selected_element = Element::Stone;
        }
        if is_key_pressed(KeyCode::Key4) {
            self.selected_element = Element::Air;
        }

        if is_key_pressed(KeyCode::Space) {
            self.paused = !self.paused;
        }

        if is_key_pressed(KeyCode::C) {
            self.clear_grid(grid);
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let grid_x = (mouse_x / self.cell_size) as usize;
            let grid_y = (mouse_y / self.cell_size) as usize;
            grid.place_element(grid_x, grid_y, self.selected_element, 2);
        }
    }

    pub fn render(&self, grid: &Grid) {
        clear_background(BLACK);

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let element = grid.get(x, y);
                if element != Element::Air {
                    draw_rectangle(
                        x as f32 * self.cell_size,
                        y as f32 * self.cell_size,
                        self.cell_size,
                        self.cell_size,
                        element.color(),
                    );
                }
            }
        }

        let ui_text = format!(
            "Selected: {:?} | 1:Sand 2:Water 3:Stone 4:Air | Space:Pause | C:Clear | FPS: {}",
            self.selected_element,
            get_fps()
        );
        draw_text(&ui_text, 10.0, 20.0, 20.0, WHITE);

        if self.paused {
            draw_text("PAUSED", 10.0, 40.0, 20.0, RED);
        }
    }

    fn clear_grid(&self, grid: &mut Grid) {
        let width = grid.width();
        let height = grid.height();

        *grid = Grid::new(width, height);

        for x in 0..width {
            grid.set(x, height - 1, Element::Stone);
        }
        for y in 0..height {
            grid.set(0, y, Element::Stone);
            grid.set(width - 1, y, Element::Stone);
        }
    }
}
