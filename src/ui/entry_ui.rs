use crossterm_window::{window::{Window, Rect}, text::Style};


#[derive(Debug)]
pub struct EntryUi {
    win: Window,
}

impl EntryUi {
    pub fn new(area: Rect) -> Self {
        EntryUi {
            win: Window::new(area),
        }
    }

    pub fn win(&self) -> &Window {
        &self.win
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.win.resize(width, height)
    }

    pub fn set_text(&mut self, s: &str) -> u16 {
        self.win.reset();
        self.win.set_stringn(0, 0, s, usize::MAX, Style::default()).0
    }
}
