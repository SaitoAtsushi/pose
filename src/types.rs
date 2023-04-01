
#[derive(Debug, PartialEq)]
pub enum PoseType {
    String(String),
    Number(f64),
    Symbol(String),
    List(Vec<PoseType>),
    End,
}

#[derive(Debug, PartialEq)]
pub enum PoseError {
    InvalidString,
    InvalidNumber,
    InvalidSymbol,
    InvalidEnd,
    NothingClosingParenthesis,
    InvalidFirstLetter,
}
