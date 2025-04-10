use crate::tokens::Tokens;

pub struct Lexer {
    scanning: Vec<char>,
    pub scanned: Vec<(Tokens, u64, u64)>,
    position: usize,
    line: u64,
    line_pos: u64
}

impl Lexer {
    pub fn new(v: Vec<char>) -> Self {
        Self {
            scanning: v,
            scanned: vec![],
            position: 0,
            line: 1,
            line_pos: 1
        }
    }

    pub fn get_scanned_vec(&self) -> Vec<(Tokens, u64, u64)> {
        self.scanned.clone()
    }
    
    pub fn get_final_pos(&self) -> (u64, u64) {
        (self.line, self.line_pos)
    }

    fn now(&self) -> char {
        self.scanning[self.position]
    }

    fn peek(&self) -> Option<char> {
        if self.position + 1 < self.scanning.len() {
            Some(self.scanning[self.position + 1])
        }
        else {
            None
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn scan(&mut self) -> Result<(), (u64, u64)> {
        self.scanning.push(' ');
        let mut temp_num: i64;
        let mut temp_str: String;
        while self.position < self.scanning.len() {
            match self.now() {
                '0'..='9' => {
                    temp_num = 0;
                    while self.now().is_numeric() {
                        temp_num = temp_num.wrapping_mul(10i64) + self.now() as i64 - '0' as i64;
                        self.advance();
                    }
                    self.scanned.push((Tokens::Integer(temp_num), self.line, self.line_pos));
                },
                '_' | 'a'..='z' | 'A'..='Z' => {
                    temp_str = String::new();
                    loop {
                        if let '_' | 'a'..='z' | 'A'..='Z' = self.now() {
                            temp_str.push(self.now());
                            self.advance();
                        }
                        else {
                            break;
                        }
                    }
                    self.scanned.push((match &temp_str[..] {
                        "print" => Tokens::Print,
                        "println" => Tokens::Println,
                        "var" => Tokens::Var,
                        "if" => Tokens::If,
                        "else" => Tokens::Else,
                        "while" => Tokens::While,
                        "or" => Tokens::LogicalOr,
                        "and" => Tokens::LogicalAnd,
                        _ => Tokens::Identifier(temp_str.clone())
                    }, self.line, self.line_pos));
                },
                '|' => {
                    if let Some('|') = self.peek() {
                        self.scanned.push((Tokens::LogicalOr, self.line, self.line_pos));
                        self.advance();
                        self.advance();
                    }
                    else { 
                        self.scanned.push((Tokens::BitwiseOr, self.line, self.line_pos));
                        self.advance();
                    }
                },
                '&' => {
                    if let Some('&') = self.peek() {
                        self.scanned.push((Tokens::LogicalAnd, self.line, self.line_pos));
                        self.advance();
                        self.advance();
                    }
                    else { 
                        self.scanned.push((Tokens::BitwiseAnd, self.line, self.line_pos));
                        self.advance();
                    }
                },
                '^' => {
                    self.scanned.push((Tokens::BitwiseXor, self.line, self.line_pos));
                    self.advance();
                },
                '>' => {
                    if let Some('=') = self.peek() {
                        self.scanned.push((Tokens::GreaterEqual, self.line, self.line_pos));
                        self.advance();
                        self.advance();
                    }
                    else { 
                        self.scanned.push((Tokens::Greater, self.line, self.line_pos));
                        self.advance();
                    }
                },
                '<' => {
                    if let Some('=') = self.peek() {
                        self.scanned.push((Tokens::LesserEqual, self.line, self.line_pos));
                        self.advance();
                        self.advance();
                    }
                    else { 
                        self.scanned.push((Tokens::Lesser, self.line, self.line_pos));
                        self.advance();
                    }
                },
                '=' => {
                    if let Some('=') = self.peek() {
                        self.scanned.push((Tokens::Equal, self.line, self.line_pos));
                        self.advance();
                        self.advance();
                    }
                    else { 
                        self.scanned.push((Tokens::Assign, self.line, self.line_pos));
                        self.advance();
                    }
                },
                '!' => {
                    if let Some('=') = self.peek() {
                        self.scanned.push((Tokens::NotEqual, self.line, self.line_pos));
                        self.advance();
                        self.advance();
                    }
                    else { 
                        self.scanned.push((Tokens::LogicalNot, self.line, self.line_pos));
                        self.advance();
                    }
                },
                '~' => {
                    self.scanned.push((Tokens::BitwiseNot, self.line, self.line_pos));
                    self.advance();
                },
                '+' => {
                    self.scanned.push((Tokens::Plus, self.line, self.line_pos));
                    self.advance();
                },
                '-' => {
                    self.scanned.push((Tokens::Minus, self.line, self.line_pos));
                    self.advance();
                },
                '*' => {
                    self.scanned.push((Tokens::Star, self.line, self.line_pos));
                    self.advance();
                },
                '/' => {
                    self.scanned.push((Tokens::Slash, self.line, self.line_pos));
                    self.advance();
                },
                '%' => {
                    self.scanned.push((Tokens::Percent, self.line, self.line_pos));
                    self.advance();
                },
                '(' => {
                    self.scanned.push((Tokens::LeftParen, self.line, self.line_pos));
                    self.advance();
                },
                ')' => {
                    self.scanned.push((Tokens::RightParen, self.line, self.line_pos));
                    self.advance();
                },
                '{' => {
                    self.scanned.push((Tokens::LeftBrace, self.line, self.line_pos));
                    self.advance();
                },
                '}' => {
                    self.scanned.push((Tokens::RightBrace, self.line, self.line_pos));
                    self.advance();
                },
                ';' => {
                    self.scanned.push((Tokens::SemiColon, self.line, self.line_pos));
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.line_pos = 0;
                    self.advance();
                }
                ' ' | '\t' | '\r' => {
                    self.advance();
                },
                _ => {
                    return Err((self.line, self.line_pos)) 
                }
            }
            self.line_pos += 1;
        }
        Ok(())
    }
}
