use crate::{tokens::Tokens, tokens::Tokens::*, nodes::*};

pub struct Parser {
    parsing: Vec<(Tokens, u64, u64)>,
    pub parsed: Vec<Box<dyn Executable>>,
    position: usize,
    end_pos: (u64, u64)
}

impl Parser {
    pub fn new(v: Vec<(Tokens, u64, u64)>, p: (u64, u64)) -> Self {
        Self {
            parsing: v,
            parsed: vec![],
            position: 0,
            end_pos: p
        }
    }

    fn now(&self) -> Tokens {
        self.parsing[self.position].0.clone()
    }

    fn now_line(&self) -> u64 {
        self.parsing[self.position].1
    }

    fn now_pos(&self) -> (u64, u64) {
        (self.parsing[self.position].1, self.parsing[self.position].2)
    }

    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn is_end(&self) -> bool {
        self.position >= self.parsing.len()
    }

    fn eof_error_check(&self) -> Result<(), (u64, u64)> {
        if self.is_end() {
            return Err(self.end_pos);
        }
        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), (u64, u64)> {
        while !self.is_end() {
            let s = self.parse_statement()?;
            self.parsed.push(s);
        }
        Ok(())
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Executable>, (u64, u64)> {
        match self.now() {
            Print => Ok(self.parse_print()?),
            Println => Ok(self.parse_println()?),
            Var => Ok(self.parse_define()?),
            Identifier(_) => Ok(self.parse_assign()?),
            If => Ok(self.parse_if()?),
            While => Ok(self.parse_while()?),
            _ => Err(self.now_pos())
        }
    }

    fn parse_print(&mut self) -> Result<Box<PrintStatement>, (u64, u64)> {
        let l = self.now_line();
        self.advance();
        let _ = self.eof_error_check()?;
        if let SemiColon = self.now() {
            self.advance();
            Ok(Box::new(PrintStatement::new(None, l)))
        }
        else {
            let expr = self.parse_logical_or()?;
            if let SemiColon = self.now() {
                self.advance();
                Ok(Box::new(PrintStatement::new(Some(expr), l)))
            }
            else {
                Err(self.now_pos())
            }
        }
    }

    fn parse_println(&mut self) -> Result<Box<PrintlnStatement>, (u64, u64)> {
        let l = self.now_line();
        self.advance();
        let _ = self.eof_error_check()?;
        if let SemiColon = self.now() {
            self.advance();
            Ok(Box::new(PrintlnStatement::new(None, l)))
        }
        else {
            let expr = self.parse_logical_or()?;
            if let SemiColon = self.now() {
                self.advance();
                Ok(Box::new(PrintlnStatement::new(Some(expr), l)))
            }
            else {
                Err(self.now_pos())
            }
        }
    }

    fn parse_define(&mut self) -> Result<Box<DefineStatement>, (u64, u64)> {
        let l = self.now_line();
        let (iden, expr);
        self.advance();
        let _ = self.eof_error_check()?;
        if let Identifier(i) = self.now() {
            iden = i;
        }
        else {
            return Err(self.now_pos());
        }
        self.advance();
        let _ = self.eof_error_check()?;
        let Assign = self.now() else {
            return Err(self.now_pos());
        };
        self.advance();
        let _ = self.eof_error_check()?;
        expr = self.parse_logical_or()?;
        if let SemiColon = self.now() {
            self.advance();
            Ok(Box::new(DefineStatement::new(iden, expr, l)))
        }
        else {
            Err(self.now_pos())
        }
    }

    fn parse_assign(&mut self) -> Result<Box<AssignStatement>, (u64, u64)> {
        let l = self.now_line();
        let iden = if let Identifier(i) = self.now() {
            i
        }
        else {
            return Err(self.now_pos());
        };
        self.advance();
        let _ = self.eof_error_check()?;
        let Assign = self.now() else {
            return Err(self.now_pos());
        };
        self.advance();
        let _ = self.eof_error_check()?;
        let expr = self.parse_logical_or()?;
        if let SemiColon = self.now() {
            self.advance();
            Ok(Box::new(AssignStatement::new(iden, expr, l)))
        }
        else {
            Err(self.now_pos())
        }
    }
    
    fn parse_if(&mut self) -> Result<Box<IfStatement>, (u64, u64)> {
        let l = self.now_line();
        self.advance();
        let _ = self.eof_error_check()?;
        let cond = self.parse_logical_or()?;
        let LeftBrace = self.now() else {
            return Err(self.now_pos());
        };
        self.advance();
        let _ = self.eof_error_check()?;
        let mut if_stmts = vec![];
        loop {
            if let RightBrace = self.now() {
                break;
            }
            if_stmts.push(self.parse_statement()?);
        }
        self.advance();
        if self.is_end() {
            return Ok(Box::new(IfStatement::new(cond, if_stmts, vec![], l)));
        }
        if let Else = self.now() {
            self.advance();
            let _ = self.eof_error_check()?;
            let LeftBrace = self.now() else {
                return Err(self.now_pos());  
            };
            self.advance();
            let _ = self.eof_error_check()?;
            let mut else_stmts = vec![];
            loop {
                if let RightBrace = self.now() {
                    break;
                }
                else_stmts.push(self.parse_statement()?);
            }
            self.advance();
            Ok(Box::new(IfStatement::new(cond, if_stmts, else_stmts, l)))
        }
        else {
            Ok(Box::new(IfStatement::new(cond, if_stmts, vec![], l)))
        }
    }
    
    fn parse_while(&mut self) -> Result<Box<WhileStatement>, (u64, u64)> {
        let l = self.now_line();
        self.advance();
        let _ = self.eof_error_check()?;
        let cond = self.parse_logical_or()?;
        let LeftBrace = self.now() else {
            return Err(self.now_pos());
        };
        self.advance();
        let _ = self.eof_error_check()?;
        let mut while_stmts = vec![];
        loop {
            if let RightBrace = self.now() {
                break;
            }
            while_stmts.push(self.parse_statement()?);
        }
        self.advance();
        Ok(Box::new(WhileStatement::new(cond, while_stmts, l)))
    }

    fn parse_logical_or(&mut self) -> Result<Expression, (u64, u64)> {
        let mut expr = self.parse_logical_and()?;
        while let LogicalOr = self.now() {
            self.advance();
            let _ = self.eof_error_check()?;
            let right = self.parse_logical_and()?;
            expr = Expression::new(expr, ExpressionType::LogicalOr, right);
        }
        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, (u64, u64)> {
        let mut expr = self.parse_bitwise_or()?;
        while let LogicalAnd = self.now() {
            self.advance();
            let _ = self.eof_error_check()?;
            let right = self.parse_bitwise_or()?;
            expr = Expression::new(expr, ExpressionType::LogicalAnd, right);
        }
        Ok(expr)
    }
    
    fn parse_bitwise_or(&mut self) -> Result<Expression, (u64, u64)> {
        let mut expr = self.parse_bitwise_xor()?;
        while let BitwiseOr = self.now() {
            self.advance();
            let _ = self.eof_error_check()?;
            let right = self.parse_bitwise_xor()?;
            expr = Expression::new(expr, ExpressionType::BitwiseOr, right);
        }
        Ok(expr)
    }

    fn parse_bitwise_xor(&mut self) -> Result<Expression, (u64, u64)> {
        let mut expr = self.parse_bitwise_and()?;
        while let BitwiseXor = self.now() {
            self.advance();
            let _ = self.eof_error_check()?;
            let right = self.parse_bitwise_and()?;
            expr = Expression::new(expr, ExpressionType::BitwiseXor, right);
        }
        Ok(expr)
    }

    fn parse_bitwise_and(&mut self) -> Result<Expression, (u64, u64)> {
        let mut expr = self.parse_equality()?;
        while let BitwiseAnd = self.now() {
            self.advance();
            let _ = self.eof_error_check()?;
            let right = self.parse_equality()?;
            expr = Expression::new(expr, ExpressionType::BitwiseAnd, right);
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, (u64, u64)> {
        let mut expr = self.parse_comparison()?;
        while let Equal | NotEqual = self.now() {
            let oper = match self.now() {
                Equal => ExpressionType::Equal,
                NotEqual => ExpressionType::NotEqual,
                _ => ExpressionType::Integer(0)
            };
            self.advance();
            let _ = self.eof_error_check()?;
            let right = self.parse_comparison()?;
            expr = Expression::new(expr, oper, right);
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, (u64, u64)> {
        let mut expr = self.parse_hyper_1()?;
        while let Greater | GreaterEqual | Lesser | LesserEqual = self.now() {
            let oper = match self.now() {
                Greater => ExpressionType::Greater,
                GreaterEqual => ExpressionType::GreaterEqual,
                Lesser => ExpressionType::Lesser,
                LesserEqual => ExpressionType::LesserEqual,
                _ => ExpressionType::Integer(0)
            };
            self.advance();
            let _ = self.eof_error_check()?;
            let right = self.parse_hyper_1()?;
            expr = Expression::new(expr, oper, right);
        }
        Ok(expr)
    }

    fn parse_hyper_1(&mut self) -> Result<Expression, (u64, u64)> {
        let mut expr = self.parse_hyper_2_remainder()?;
        while let Plus | Minus = self.now() {
            let oper = match self.now() {
                Plus => ExpressionType::Plus,
                Minus => ExpressionType::Minus,
                _ => ExpressionType::Integer(0)
            };
            self.advance();
            let _ = self.eof_error_check()?;
            let right = self.parse_hyper_2_remainder()?;
            expr = Expression::new(expr, oper, right);
        }
        Ok(expr)
    }

    fn parse_hyper_2_remainder(&mut self) -> Result<Expression, (u64, u64)> {
        let mut expr = self.parse_unary()?;
        while let Star | Slash | Percent = self.now() {
            let oper = match self.now() {
                Star => ExpressionType::Multiply,
                Slash => ExpressionType::Divide,
                Percent => ExpressionType::Remainder,
                _ => ExpressionType::Integer(0)
            };
            self.advance();
            let _ = self.eof_error_check()?;
            let right = self.parse_unary()?;
            expr = Expression::new(expr, oper, right);
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, (u64, u64)> {
        let _ = self.eof_error_check()?;
        if let LogicalNot | BitwiseNot | Minus = self.now() {
            let oper = match self.now() {
                LogicalNot => ExpressionType::LogicalNot,
                BitwiseNot => ExpressionType::BitwiseNot,
                Minus => ExpressionType::UnaryMinus,
                _ => ExpressionType::Integer(0)
            };
            self.advance();
            let _ = self.eof_error_check()?;
            let right = self.parse_unary()?;
            return Ok(Expression::new_unary(oper, right));
        }
        Ok(self.parse_primary()?)
    }

    fn parse_primary(&mut self) -> Result<Expression, (u64, u64)> {
        let _ = self.eof_error_check()?;
        match self.now() {
            Integer(i) => {
                self.advance();
                Ok(Expression::new_empty(ExpressionType::Integer(i)))
            },
            LeftParen => {
                self.advance();
                let _ = self.eof_error_check()?;
                let expr = self.parse_logical_or()?;
                let _ = self.eof_error_check()?;
                if let RightParen = self.now() {
                    self.advance();
                    Ok(Expression::new_empty(ExpressionType::Parentheses(Box::new(expr))))
                }
                else {
                    Err(self.now_pos())
                }
            },
            Identifier(i) => {
                self.advance();
                Ok(Expression::new_empty(ExpressionType::Variable(i)))
            },
            _ => Err(self.now_pos())
        }
    }
}
