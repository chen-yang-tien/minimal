#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Print, Println, Var, If, Else, While, Identifier(String),
    Integer(i64), LogicalOr, BitwiseOr, LogicalAnd, BitwiseAnd, BitwiseXor,
    Greater, GreaterEqual, Lesser, LesserEqual, Equal, Assign, NotEqual, BitwiseNot, LogicalNot,
    Plus, Minus, Star, Slash, Percent, LeftParen, RightParen, LeftBrace, RightBrace, SemiColon
}
