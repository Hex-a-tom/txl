use std::io;

mod parser;

use crate::model::sheet::Sheet;
use crate::ui::Ui;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use self::parser::parse;

#[derive(Debug)]
pub struct Controller {
    ui: Ui,
    sheet: Sheet,
    entry: String,
    selection: (usize, usize),
}

impl Controller {
    pub fn new() -> Self {
        let ui = Ui::init().expect("Unable to initialize ui");
        let sheet = Sheet::new();
        let entry = "".to_string();

        Controller {
            ui,
            sheet,
            entry,
            selection: (0, 0),
        }
    }

    pub fn set_entry(&mut self) -> io::Result<()> {
        self.entry.clear();
        self.sheet[self.selection.0][self.selection.1]
            .entry(&mut self.entry)
            .unwrap();
        self.ui.set_entry(&self.entry)
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.ui.redraw(&self.sheet).expect("Unable to draw ui");

        let mut running = true;

        while running {
            match crossterm::event::read()? {
                crossterm::event::Event::FocusGained => todo!(),
                crossterm::event::Event::FocusLost => todo!(),
                crossterm::event::Event::Key(k) => match k {
                    KeyEvent {
                        modifiers: KeyModifiers::CONTROL,
                        code: KeyCode::Char('c'),
                        ..
                    } => running = false,
                    KeyEvent { code, .. } => match code {
                        KeyCode::Up => {
                            self.selection.1 = self.selection.1.saturating_sub(1);
                            self.ui.set_selection(
                                self.selection.0,
                                self.selection.1,
                                &self.sheet,
                            )?;
                            self.set_entry()?;
                        }
                        KeyCode::Down => {
                            self.selection.1 += 1;
                            self.ui.set_selection(
                                self.selection.0,
                                self.selection.1,
                                &self.sheet,
                            )?;
                            self.set_entry()?;
                        }
                        KeyCode::Right => {
                            self.selection.0 += 1;
                            self.ui.set_selection(
                                self.selection.0,
                                self.selection.1,
                                &self.sheet,
                            )?;
                            self.set_entry()?;
                        }
                        KeyCode::Left => {
                            self.selection.0 = self.selection.0.saturating_sub(1);
                            self.ui.set_selection(
                                self.selection.0,
                                self.selection.1,
                                &self.sheet,
                            )?;
                            self.set_entry()?;
                        }
                        KeyCode::Enter => {
                            self.sheet[self.selection.0][self.selection.1] = parse(&self.entry);
                            self.ui.redraw(&self.sheet)?;
                        }
                        KeyCode::Backspace => {
                            self.entry.pop();
                            self.ui.set_entry(&self.entry)?;
                        }
                        KeyCode::Char(ch) => {
                            self.entry.push(ch);
                            self.ui.set_entry(&self.entry)?;
                        }
                        _ => (),
                    },
                },
                crossterm::event::Event::Mouse(_) => todo!(),
                crossterm::event::Event::Paste(_) => todo!(),
                crossterm::event::Event::Resize(width, height) => {
                    self.ui.resize(width, height, &self.sheet)?
                }
            }
        }

        self.ui.finish().expect("Unable to finish ui");

        Ok(())
    }
}
