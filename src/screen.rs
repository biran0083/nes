pub const SCREEN_HEIGHT: usize = 240;
pub const SCREEN_WIDTH: usize = 256;

pub struct ScreenState {
    screen_state: [u8; SCREEN_HEIGHT * SCREEN_WIDTH * 3],
}

impl ScreenState {
    pub fn new() -> Self {
        ScreenState {
            screen_state: [0; SCREEN_HEIGHT * SCREEN_WIDTH * 3],
        }
    }

    pub fn update(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        let idx = y * SCREEN_WIDTH * 3 + x * 3;
        self.screen_state[idx] = color.0;
        self.screen_state[idx + 1] = color.1;
        self.screen_state[idx + 2] = color.2;
    }

    pub fn get_picxel_data(&self) -> &[u8] {
        self.screen_state.as_ref()
    }
}
