use std::rc::Rc;

use crate::model::{sheet::{Cell, Expression}, calc};

pub fn parse(s: &str) -> Cell {
    // TODO: Impement back the non expression types
    match s.chars().next() {
        Some(c) => if c == '=' {
            let ex = Expression::new(s.to_owned());
            Cell::Expression(Rc::new(ex), Err(calc::ExecutionError::NotExecuted))
        } else {
            match s.parse() {
                Ok(o) => Cell::Val(o),
                Err(_) => Cell::String(s.to_string()),
            }
        },
        None => Cell::None,
    }
}
