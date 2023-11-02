use super::sheet::Sheet;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Err,
    Add,
    Sub,
    Mul,
    Div,
    Cell(usize, usize),
    Val(i64),
}

impl Node {
    fn precedence(&self) -> u8 {
        match self {
            Node::Err => 0,
            Node::Add => 4,
            Node::Sub => 4,
            Node::Mul => 3,
            Node::Div => 3,
            Node::Cell(_, _) => 0,
            Node::Val(_) => 0,
        }
    }

    fn is_op(&self) -> bool {
        match self {
            Node::Err => false,
            Node::Cell(_, _) => false,
            Node::Val(_) => false,
            _ => true,
        }
    }

    fn eval(&self) -> Result<i64, ExecutionError> {
        match self {
            Node::Cell(_, _) => todo!(),
            Node::Val(v) => Ok(*v),
            _ => Err(ExecutionError::NotImpemented),
        }
    }

    // TODO: Combine eval and calc
    fn calc(&self, stack: &mut Vec<i64>) -> Result<i64, ExecutionError> {
        use ExecutionError::OutOfStack;
        match self {
            Node::Add => Ok(stack.pop().ok_or(OutOfStack)? + stack.pop().ok_or(OutOfStack)?),
            Node::Sub => {
                let sub = stack.pop().ok_or(OutOfStack)?;
                Ok(stack.pop().ok_or(OutOfStack)? - sub)
            }
            Node::Mul => Ok(stack.pop().ok_or(OutOfStack)? * stack.pop().ok_or(OutOfStack)?),
            Node::Div => {
                let nom = stack.pop().ok_or(OutOfStack)?;
                Ok(stack.pop().ok_or(OutOfStack)? / nom)
            }
            _ => Err(ExecutionError::NotImpemented),
        }
    }

    fn gt(&self, other: &Self) -> bool {
        self.precedence() < other.precedence()
    }
}

pub fn parse(expr: &str) -> ByteCode {
    let mut expression = vec![];
    let mut stack: Vec<Node> = vec![];

    for n in Lexer::new(expr) {
        if n == Node::Err {
            return ByteCode { code: Err(()) };
        } else if !n.is_op() {
            expression.push(n);
        } else {
            while let Some(s) = stack.last() {
                if s.gt(&n) {
                    expression.push(stack.pop().unwrap())
                } else {
                    break;
                }
            }
            stack.push(n)
        }
    }
    while let Some(s) = stack.pop() {
        expression.push(s)
    }

    ByteCode {
        code: Ok(expression),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExecutionError {
    CompilationError,
    NotImpemented,
    SyntaxError,
    OutOfStack,
    DivByZero,
    NotExecuted,
    Cyclic,
    CellNotFound,
}

#[derive(Debug, Clone)]
pub struct ByteCode {
    code: Result<Vec<Node>, ()>,
}

impl ByteCode {
    pub fn execute(&self, sheet: &Sheet) -> Result<i64, ExecutionError> {
        match &self.code {
            Ok(expr) => {
                let mut stack = vec![];

                for n in expr {
                    if let Node::Cell(x, y) = n {
                        if let Some(c) = sheet[*x][*y].val() {
                            stack.push(c);
                        } else {
                            return Err(ExecutionError::CellNotFound);
                        }
                    } else if n.is_op() {
                        let v = n.calc(&mut stack)?;
                        stack.push(v);
                    } else {
                        stack.push(n.eval()?)
                    }
                }

                stack.last().ok_or(ExecutionError::OutOfStack).copied()
            }
            Err(e) => Err(ExecutionError::CompilationError),
        }
    }

    pub fn deps(&self, pos: (u16, u16)) -> Vec<(u16, u16)> {
        match &self.code {
            Ok(expr) => expr
                .iter()
                .filter_map(|c| {
                    if let Node::Cell(x, y) = c {
                        Some((*x as u16, *y as u16))
                    } else {
                        None
                    }
                })
                .collect(),
            Err(e) => vec![],
        }
    }
}

#[derive(Debug)]
struct Lexer<'a> {
    s: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Lexer { s }
    }
}

static LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl<'a> Iterator for Lexer<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        use Node::*;

        self.s = self.s.trim_start();

        let c = self.s.chars().next();
        if let Some(ch) = c {
            if ch.is_ascii_digit() {
                let mut len = self.s.len();
                for (i, ch) in self.s.chars().enumerate() {
                    if !ch.is_ascii_digit() {
                        len = i;
                        break;
                    }
                }
                let (num, rest) = self.s.split_at(len);
                self.s = rest;
                return Some(num.parse().map_or(Err, |x| Val(x)));
            } else if ch == '+' {
                self.s = &self.s[1..];
                return Some(Add);
            } else if ch == '-' {
                self.s = &self.s[1..];
                return Some(Sub);
            } else if ch == '*' {
                self.s = &self.s[1..];
                return Some(Mul);
            } else if ch == '/' {
                self.s = &self.s[1..];
                return Some(Div);
            } else if let Some(index) = LETTERS.iter().position(|x| *x == ch) {
                let row = self.s.chars().nth(1).unwrap().to_digit(10).unwrap();
                self.s = &self.s[2..];
                return Some(Cell(index, row as usize - 1));
            }
            Some(Err)
        } else {
            None
        }
    }
}
