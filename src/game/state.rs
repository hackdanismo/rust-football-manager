#[derive(Debug)]
pub struct GameState {
    pub current_day: u32,
    pub season: u32,
    pub running: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_day: 1,
            season: 2026,
            running: true,
        }
    }

    pub fn advance_day(&mut self) {
        self.current_day += 1;
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}