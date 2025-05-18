pub enum Mode {
    Default,
}

pub struct Config {
    mode: Mode,
}

impl Config {
    pub fn new(mode: Mode) -> Self {
        Self { mode }
    }

    pub fn get_display_items(&self) -> Vec<(&'static str, String)> {
        let mode_display = match self.mode {
            Mode::Default => "Default",
        };
        vec![("Mode", mode_display.to_string())]
    }
}
