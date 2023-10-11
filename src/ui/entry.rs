use crossterm_window::{
    text::Style,
    window::{Rect, Window},
};

#[derive(Debug)]
pub struct Entry {
    text: String,
}

impl Entry {

    pub fn input(&mut self, ch: char) -> u16 {
        self.text.push(ch);
        let mut tmp = [0u8; 4];
        let str = ch.encode_utf8(&mut tmp);
        self.win.set_stringn(self.pos, 0, str, 1, Style::default());
        self.pos += 1;
        self.pos
    }

    pub fn backspace(&mut self) -> u16 {
            self.text.pop();
            if self.pos > 0 {
            self.win
                .set_stringn(self.pos - 1, 0, " ", 1, Style::default());
            self.pos -= 1;
        }
        self.pos
    }

}
