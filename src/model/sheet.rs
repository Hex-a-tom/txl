use std::{fmt::Display, ops::{IndexMut, Index}, collections::{HashMap, HashSet}, rc::Rc};

use super::calc::{parse, ExecutionError, ByteCode};

#[derive(Debug, Clone)]
pub struct Expression {
    text: String,
    run: ByteCode,
}

impl Expression {
    pub fn new(text: String) -> Self {
        let run = parse(&text[1..]);
        Expression { text, run }
    }

    /// Get the cells this expression depends on for the position of `self_pos`
    pub fn deps(&self, self_pos: Pos) -> Vec<Pos> {
        self.run.deps(self_pos)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    None,
    Val(i64),
    String(String),
    Expression(Rc<Expression>, Result<i64, ExecutionError>),
}

impl Cell {
    pub fn justify_right(&self) -> bool {
        match self {
            Cell::None => true,
            Cell::Val(_) => true,
            Cell::String(_) => false,
            Cell::Expression(_, _) => true,
        }
    }

    pub fn entry(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        match self {
            Cell::None => Ok(()),
            Cell::Val(v) => write!(f, "{}", v),
            Cell::String(s) => write!(f, "{}", s),
            Cell::Expression(e, _) => write!(f, "{}", e),
        }
    }

    pub fn val(&self) -> Option<i64> {
        match self {
            Cell::None => None,
            Cell::Val(v) => Some(*v),
            Cell::String(_) => None,
            Cell::Expression(_, r) => r.ok(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::None => write!(f, "---"),
            Cell::Val(v) => write!(f, "{}", v),
            Cell::String(s) => write!(f, "{}", s),
            Cell::Expression(_, r) => {
                if let Ok(v) = r {
                    write!(f, "{}", v)
                } else {
                    write!(f, "#Error")
                }
            },
        }
    }
}

type Pos = (u16, u16);

#[derive(Debug)]
pub struct Sheet {
    pub fields: Vec<(u16, Vec<Cell>)>,
    deps: HashMap<Pos, Vec<Pos>>
}

impl Sheet {
    pub fn new() -> Self {
        Sheet { fields: vec![(7, vec![Cell::None; 30]); 10], deps: HashMap::new() }
    }

    /// Inserts and calculates the value of the `cell` at position `pos`
    pub fn insert_cell(&mut self, mut cell: Cell, pos: (usize, usize)) {
        // TODO: <- Remove old cell if it exists
        if let Cell::Expression(ex, ref mut res) = &mut cell {
            if self.is_cyclic((pos.0 as u16, pos.1 as u16)) {
                *res = Err(ExecutionError::Cyclic);
                self[pos.0][pos.1] = cell;
            } else {
                for d in ex.deps((pos.0 as u16, pos.1 as u16)) {
                    self.add_dependency(d, (pos.0 as u16, pos.1 as u16));
                }
                *res = ex.run.execute(&self);
                self[pos.0][pos.1] = cell;
            }
        }
    }

    fn is_cyclic(&self, pos: Pos) -> bool {
        let mut next: Vec<Pos> = vec![];
        let mut been_at: HashSet<Pos> = HashSet::new();

        if let Some(xc) = self.deps.get(&pos) {
            for i in xc {
                next.push(*i);
            }
        }

        while let Some(this) = next.pop() {
            if this == pos {
                return true;
            }

            been_at.insert(this);

            if let Some(xc) = self.deps.get(&this) {
                for i in xc {
                    if !been_at.contains(i) {
                        next.push(*i);
                    }
                }
            }
        }

        false
    }

    fn add_dependency(&mut self, target: Pos, depender: Pos) {
        if let Some(this) = self.deps.get_mut(&target) {
            this.push(depender);
        } else {
            self.deps.insert(target, vec![depender]);
        }
    }

    fn remove_dependency(&mut self, target: Pos, depender: Pos) {
        if let Some(this) = self.deps.get_mut(&target) {
            let index = this.iter().position(|x| *x == depender).unwrap();
            this.remove(index);
        }
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
