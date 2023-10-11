use super::sheet::Sheet;

#[derive(Debug, PartialEq)]
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

    fn eval(&self) -> Result<i64, ()> {
        match self {
            Node::Cell(_, _) => todo!(),
            Node::Val(v) => Ok(*v),
            _ => Err(())
        }
    }

    // TODO: Combine eval and calc
    fn calc(&self, stack: &mut Vec<i64>) -> Result<i64, ()> {
        match self{
            Node::Add => Ok(stack.pop().ok_or(())? + stack.pop().ok_or(())?),
            Node::Sub => {
                let sub = stack.pop().ok_or(())?;
                Ok(stack.pop().ok_or(())? - sub)
            } ,
            Node::Mul => Ok(stack.pop().ok_or(())? * stack.pop().ok_or(())?),
            Node::Div => {
                let nom = stack.pop().ok_or(())?;
                Ok(stack.pop().ok_or(())? / nom)
            },
            _ => Err(())
        }
    }

    fn gt(&self, other: &Self) -> bool {
        self.precedence() < other.precedence()
    }
}

pub fn parse(expr: &str) -> Result<Vec<Node>, ()> {
    let mut expression = vec![];
    let mut stack: Vec<Node> = vec![];

    for n in Lexer::new(expr) {
        if n == Node::Err {
            return Err(());
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

    Ok(expression)
}

pub fn execute(expr: Vec<Node>, sheet: Sheet) -> Result<i64, ()>{
    let mut stack = vec![];

    for n in expr {
        if n.is_op() {
            let v = n.calc(&mut stack)?;
            stack.push(v);
        } else {
            stack.push(n.eval()?)
        }
    }

    stack.last().ok_or(()).copied()
}

#[derive(Debug)]
struct Lexer<'a> {
    s: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Lexer {
            s
        }
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
                return Some(Add)
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
                let row = self.s.chars().nth(2).unwrap().to_digit(10).unwrap();
                self.s = &self.s[2..];
                return Some(Cell(index, row as usize));
            }
            Some(Err)
        } else {
            None
        }
    }
}
