use std::io;

use std::fmt::Write;
mod entry_ui;

use crate::model::sheet::Sheet;
use crossterm::{
    style::Color,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm_window::{
    terminal::Terminal,
    text::Style,
    window::{Rect, Window},
};
use unicode_width::UnicodeWidthStr;

use self::entry_ui::EntryUi;

#[derive(Debug)]
pub struct Ui {
    terminal: Terminal,
    sheet: SheetUi,
    entry: EntryUi,
}

impl Ui {
    pub fn init() -> io::Result<Self> {
        crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;

        let (width, height) = crossterm::terminal::size().expect("Failed to get terminal size");

        let term = Terminal::init();
        let sheet = SheetUi::new(Rect::new(0, 1, width, height - 2));
        let entry = EntryUi::new(Rect::new(0, 0, width, 1));

        let ui = Ui {
            terminal: term,
            sheet,
            entry,
        };

        Ok(ui)
    }

    pub fn finish(&self) -> io::Result<()> {
        crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()
    }

    pub fn redraw(&mut self, sheet: &Sheet) -> io::Result<()> {
        self.sheet.redraw(sheet);

        self.terminal.put(&self.sheet.win)?;
        self.terminal.put(self.entry.win())?;

        Ok(())
    }

    pub fn resize(&mut self, width: u16, height: u16, sheet: &Sheet) -> io::Result<()> {
        self.sheet.win.resize(width, height-3);
        self.sheet.redraw(sheet);
        self.entry.resize(width, height);
        self.terminal.resize(width, height);
        self.terminal.put(&self.sheet.win)?;
        self.terminal.put(self.entry.win())
    }

    pub fn set_selection(&mut self, x: usize, y: usize, sheet: &Sheet) -> io::Result<()> {
        self.sheet.selection = (x, y);

        self.sheet.draw_selection(sheet);

        self.terminal.put(&self.sheet.win)?;

        Ok(())
    }

    pub fn set_entry(&mut self, s: &str) -> io::Result<()> {
        let x = self.entry.set_text(s);
        self.terminal.put(self.entry.win())?;
        Terminal::set_cursor_pos(x, 0)
    }
}

#[derive(Debug)]
struct SheetUi {
    win: Window,
    selection: (usize, usize),
}

impl SheetUi {
    pub fn new(area: Rect) -> Self {
        SheetUi {
            win: Window::new(area),
            selection: (0, 0),
        }
    }

    pub fn redraw(&mut self, sheet: &Sheet) {
        self.win.reset();
        self.draw_sheet(sheet);
        self.draw_selection(sheet);
    }

    fn draw_selection(&mut self, sheet: &Sheet) {
        self.win.set_style(
            Rect::new(3, 1, self.win.width() - 3, self.win.height() - 1),
            Style::reset(),
        );
        self.win.set_style(
            Rect::new(0, 0, self.win.width(), 1),
            Style::default().bg(Color::Cyan).fg(Color::Black),
        );
        self.win.set_style(
            Rect::new(0, 0, 3, self.win.height()),
            Style::default().bg(Color::Cyan).fg(Color::Black),
        );

        let mut offset = 3;
        for (x, row) in sheet.fields.iter().enumerate() {
            if offset > self.win.width() {
                break;
            }

            if self.selection.0 == x {
                self.win.set_style(
                    Rect::new(offset, 0, row.0, 1),
                    Style::default().bg(Color::DarkBlue).fg(Color::White),
                );
                self.win.set_style(
                    Rect::new(offset, self.selection.1 as u16 + 1, row.0, 1),
                    Style::default().fg(Color::Black).bg(Color::Cyan),
                );
                break;
            }

            offset += row.0;
        }
        self.win.set_style(
            Rect::new(0, self.selection.1 as u16 + 1, 3, 1),
            Style::default().bg(Color::DarkBlue).fg(Color::White),
        );
    }

    fn draw_sheet(&mut self, sheet: &Sheet) {
        let mut offset = 3;
        for (x, row) in sheet.fields.iter().enumerate() {
            if offset > self.win.width() {
                break;
            }

            self.win
                .set_stringn(offset + row.0 / 2, 0, num_to_row(x), 1, Style::default());

            let mut s = "".to_string();

            for (y, cell) in row.1.iter().enumerate() {
                s.clear();
                write!(s, "{}", cell).unwrap();
                if cell.justify_right() {
                    let length = UnicodeWidthStr::width(&s[..]);
                    self.win.set_stringn(
                        offset + row.0.saturating_sub(length as u16 + 1),
                        y as u16 + 1,
                        &s,
                        length,
                        Style::default(),
                    );
                } else {
                    self.win.set_stringn(
                        offset,
                        y as u16 + 1,
                        &s,
                        row.0 as usize - 1,
                        Style::default(),
                    );
                }
            }

            offset += row.0;
        }

        for i in 1..self.win.height() {
            self.win
                .set_stringn(0, i, format!("{}", i), 3, Style::default());
        }
    }
}

static LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn num_to_row(row: usize) -> String {
    LETTERS[row].to_string()
}
