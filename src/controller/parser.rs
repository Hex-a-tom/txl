use crate::model::{sheet::Cell, calc};

pub fn parse(s: &str) -> Cell {
    match s.chars().next() {
        Some(c) => if c == '=' {
            let par = calc::parse(&s[1..]);
            match par {
                Ok(o) => Cell::Expression(s.to_string(), 0),
                Err(_) => Cell::ErrExpression(s.to_string()),
            }
        } else {
            match s.parse() {
                Ok(o) => Cell::Val(o),
                Err(_) => Cell::String(s.to_string()),
            }
        },
        None => Cell::None,
    }
}
