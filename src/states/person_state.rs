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
