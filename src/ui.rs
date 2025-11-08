use crate::element::Element;
use crate::grid::Grid;
use macroquad::prelude::*;

const BUTTON_HEIGHT: f32 = 50.0;
const BUTTON_PADDING: f32 = 10.0;
const UI_AREA_HEIGHT: f32 = 80.0;

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

    fn get_game_area_start_y(&self) -> f32 {
        UI_AREA_HEIGHT
    }

    fn get_button_rect(&self, index: usize, total_buttons: usize) -> (f32, f32, f32, f32) {
        let screen_w = screen_width();
        let total_padding = BUTTON_PADDING * (total_buttons as f32 + 1.0);
        let button_width = (screen_w - total_padding) / total_buttons as f32;

        let x = BUTTON_PADDING + index as f32 * (button_width + BUTTON_PADDING);
        let y = UI_AREA_HEIGHT - BUTTON_HEIGHT - BUTTON_PADDING;

        (x, y, button_width, BUTTON_HEIGHT)
    }

    fn is_point_in_rect(&self, px: f32, py: f32, x: f32, y: f32, w: f32, h: f32) -> bool {
        px >= x && px <= x + w && py >= y && py <= y + h
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
            self.selected_element = Element::Person;
        }
        if is_key_pressed(KeyCode::Key5) {
            self.selected_element = Element::Air;
        }

        if is_key_pressed(KeyCode::Space) {
            self.paused = !self.paused;
        }

        if is_key_pressed(KeyCode::C) {
            self.clear_grid(grid);
        }

        let (mouse_x, mouse_y) = mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) {
            let elements = Element::all();
            for (i, element) in elements.iter().enumerate() {
                let (bx, by, bw, bh) = self.get_button_rect(i, elements.len());
                if self.is_point_in_rect(mouse_x, mouse_y, bx, by, bw, bh) {
                    self.selected_element = *element;
                    return;
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            let game_area_start_y = self.get_game_area_start_y();
            if mouse_y >= game_area_start_y {
                let grid_x = (mouse_x / self.cell_size) as usize;
                let grid_y = ((mouse_y - game_area_start_y) / self.cell_size) as usize;
                grid.place_element(grid_x, grid_y, self.selected_element, 2);
            }
        }
    }

    pub fn render(&self, grid: &Grid) {
        clear_background(BLACK);

        let game_area_start_y = self.get_game_area_start_y();

        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            UI_AREA_HEIGHT,
            Color::new(0.15, 0.15, 0.15, 1.0),
        );

        let title = "POWDER LIST";
        let title_y = 20.0;

        draw_text(title, 10.0, title_y, 20.0, WHITE);

        let elements = Element::all();
        for (i, element) in elements.iter().enumerate() {
            let (x, y, w, h) = self.get_button_rect(i, elements.len());

            let bg_color = if *element == self.selected_element {
                Color::new(0.3, 0.3, 0.3, 1.0)
            } else {
                Color::new(0.2, 0.2, 0.2, 1.0)
            };
            draw_rectangle(x, y, w, h, bg_color);

            let preview_size = h - 20.0;
            let preview_x = x + 10.0;
            let preview_y = y + 10.0;
            draw_rectangle(preview_x, preview_y, preview_size, preview_size, element.color());

            if *element == Element::Air {
                draw_rectangle_lines(preview_x, preview_y, preview_size, preview_size, 2.0, GRAY);
            }

            let text_x = preview_x + preview_size + 10.0;
            let text_y = y + h / 2.0 + 5.0;
            draw_text(element.name(), text_x, text_y, 20.0, WHITE);

            let border_color = if *element == self.selected_element {
                YELLOW
            } else {
                GRAY
            };
            draw_rectangle_lines(x, y, w, h, 2.0, border_color);
        }

        let separator_y = UI_AREA_HEIGHT;
        draw_line(0.0, separator_y, screen_width(), separator_y, 2.0, GRAY);

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let element = grid.get(x, y);
                if element != Element::Air {
                    let pixel_x = x as f32 * self.cell_size;
                    let pixel_y = y as f32 * self.cell_size + game_area_start_y;

                    if element == Element::Person {
                        // 사람 모양으로 그리기
                        let in_water = grid.get_person_state(x, y)
                            .map(|s| s.water_ticks > 0)
                            .unwrap_or(false);
                        self.draw_person(pixel_x, pixel_y, in_water);
                    } else {
                        draw_rectangle(
                            pixel_x,
                            pixel_y,
                            self.cell_size,
                            self.cell_size,
                            element.color(),
                        );
                    }
                }
            }
        }

        let ui_text = format!(
            "Space: Pause | C: Clear | FPS: {}",
            get_fps()
        );
        let text_width = measure_text(&ui_text, None, 20, 1.0).width;
        draw_text(&ui_text, screen_width() - text_width - 10.0, 20.0, 20.0, WHITE);

        if self.paused {
            draw_text("PAUSED", screen_width() - 100.0, 40.0, 20.0, RED);
        }
    }

    fn draw_person(&self, x: f32, y: f32, in_water: bool) {
        let size = self.cell_size;

        // 물에 잠기면 빨간색으로 하이라이트
        let skin_color = if in_water {
            Color::new(1.0, 0.3, 0.3, 1.0) // 빨간색
        } else {
            Color::new(1.0, 0.8, 0.6, 1.0) // 살색
        };
        let cloth_color = if in_water {
            Color::new(0.8, 0.2, 0.2, 1.0) // 어두운 빨간색
        } else {
            Color::new(0.2, 0.4, 0.9, 1.0) // 파란색
        };

        // 머리 (상단 1/3)
        draw_circle(x + size / 2.0, y + size * 0.25, size * 0.3, skin_color);

        // 몸통 (중간)
        draw_rectangle(
            x + size * 0.25,
            y + size * 0.4,
            size * 0.5,
            size * 0.35,
            cloth_color,
        );

        // 다리 (하단)
        draw_line(
            x + size * 0.35,
            y + size * 0.75,
            x + size * 0.35,
            y + size,
            size * 0.2,
            cloth_color,
        );
        draw_line(
            x + size * 0.65,
            y + size * 0.75,
            x + size * 0.65,
            y + size,
            size * 0.2,
            cloth_color,
        );

        // 팔
        draw_line(
            x + size * 0.25,
            y + size * 0.5,
            x,
            y + size * 0.6,
            size * 0.15,
            skin_color,
        );
        draw_line(
            x + size * 0.75,
            y + size * 0.5,
            x + size,
            y + size * 0.6,
            size * 0.15,
            skin_color,
        );
    }

    fn clear_grid(&self, grid: &mut Grid) {
        let width = grid.width();
        let height = grid.height();

        *grid = Grid::new(width, height);

        for x in 0..width {
            grid.set(x, 0, Element::Stone);
            grid.set(x, height - 1, Element::Stone);
        }
        for y in 0..height {
            grid.set(0, y, Element::Stone);
            grid.set(width - 1, y, Element::Stone);
        }
    }
}
