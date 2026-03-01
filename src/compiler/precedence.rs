#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum Precendence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary
}

impl Precendence {
    pub fn next(&self) -> Self {
        match *self {
            Precendence::None => Precendence::Assignment,
            Precendence::Assignment => Precendence::Or,
            Precendence::Or => Precendence::And,
            Precendence::And => Precendence::Equality,
            Precendence::Equality => Precendence::Comparison,
            Precendence::Comparison => Precendence::Term,
            Precendence::Term => Precendence::Factor,
            Precendence::Factor => Precendence::Unary,
            Precendence::Unary => Precendence::Call,
            Precendence::Call => Precendence::Primary,
            Precendence::Primary => Precendence::Primary,
        }
    }
}


