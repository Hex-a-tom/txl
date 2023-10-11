use std::{fmt::Display, ops::{IndexMut, Index}};


#[derive(Debug, Clone)]
pub enum Cell {
    None,
    Val(i64),
    String(String),
    Expression(String, i64),
    ErrExpression(String),
}

impl Cell {
    pub fn justify_right(&self) -> bool {
        match self {
            Cell::None => true,
            Cell::Val(_) => true,
            Cell::String(_) => false,
            Cell::Expression(_, _) => true,
            Cell::ErrExpression(_) => true,
        }
    }

    pub fn entry(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        match self {
            Cell::None => Ok(()),
            Cell::Val(v) => write!(f, "{}", v),
            Cell::String(s) => write!(f, "{}", s),
            Cell::Expression(e, _) => write!(f, "{}", e),
            Cell::ErrExpression(e) => write!(f, "{}", e),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::None => write!(f, "---"),
            Cell::Val(v) => write!(f, "{}", v),
            Cell::String(s) => write!(f, "{}", s),
            Cell::Expression(_, r) => write!(f, "{}", r),
            Cell::ErrExpression(_) => write!(f, "#Error"),
        }
    }
}

#[derive(Debug)]
pub struct Sheet {
    pub fields: Vec<(u16, Vec<Cell>)>
}

impl Sheet {
    pub fn new() -> Self {
        Sheet { fields: vec![(7, vec![Cell::None; 30]); 10] }
    }
}

impl Index<usize> for Sheet {
    type Output = Vec<Cell>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields.index(index).1
    }
}

impl IndexMut<usize> for Sheet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.fields.index_mut(index).1
    }
}
