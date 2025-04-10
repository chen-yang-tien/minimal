use std::collections::HashMap;

#[derive(Debug)]
pub enum ExpressionType {
    LogicalOr,
    LogicalAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Equal, NotEqual,
    Lesser, LesserEqual, Greater, GreaterEqual,
    Plus, Minus,
    Multiply, Divide, Remainder,
    LogicalNot, BitwiseNot, UnaryMinus,
    Integer(i64), Variable(String), Parentheses(Box<Expression>)
}

#[derive(Debug)]
pub struct Expression {
    expression_type: ExpressionType,
    value1: Option<Box<Expression>>,
    value2: Option<Box<Expression>>
}

impl Expression {
    pub fn evaluate(&self, variables: &HashMap<&String, i64>) -> Result<i64, String> {
        use ExpressionType::*;

        if let Integer(_) | Variable(_) | Parentheses(_) = self.expression_type {
            return match &self.expression_type {
                Integer(i) => Ok(*i),
                Variable(s) => if let Some(i) = variables.get(&s) { Ok(*i) } else { Err(format!("'{}' Undefined variable error", s)) },
                Parentheses(e) => e.evaluate(variables),
                _ => Err("This should not happen".to_string())
            };
        }

        if let LogicalNot | BitwiseNot | UnaryMinus = self.expression_type {
            let v = self.value2.as_ref().unwrap().evaluate(variables)?;
            return match self.expression_type {
                LogicalNot => if v == 0 { Ok(1) } else { Ok(0) },
                BitwiseNot => Ok(!v),
                UnaryMinus => Ok(-v),
                _ => Err("This should not happen".to_string())
            };
        }

        let v1 = self.value1.as_ref().unwrap().evaluate(variables)?;
        let v2 = self.value2.as_ref().unwrap().evaluate(variables)?;
        match self.expression_type {
            LogicalOr => if v1 != 0 || v2 != 0 { Ok(1) } else { Ok(0) },
            LogicalAnd => if v1 != 0 && v2 != 0 { Ok(1) } else { Ok(0) },
            BitwiseOr => Ok(v1 | v2),
            BitwiseXor => Ok(v1 ^ v2),
            BitwiseAnd => Ok(v1 & v2),
            Equal => if v1 == v2 { Ok(1) } else { Ok(0) },
            NotEqual => if v1 != v2 { Ok(1) } else { Ok(0) },
            Lesser => if v1 < v2 { Ok(1) } else { Ok(0) },
            LesserEqual => if v1 <= v2 { Ok(1) } else { Ok(0) },
            Greater => if v1 > v2 { Ok(1) } else { Ok(0) },
            GreaterEqual => if v1 >= v2 { Ok(1) } else { Ok(0) },
            Plus => Ok(v1.wrapping_add(v2)),
            Minus => Ok(v1.wrapping_sub(v2)),
            Multiply => Ok(v1.wrapping_mul(v2)),
            Divide => if v2 == 0 { Err("Zero division error".to_string()) } else { Ok(v1.wrapping_div(v2)) },
            Remainder => Ok(v1 % v2),
            _ => Err("This should not happen".to_string())
        }
    }
    
    pub fn new(left: Expression, operator: ExpressionType, right: Expression) -> Self {
        Self {
            expression_type: operator,
            value1: Some(Box::new(left)),
            value2: Some(Box::new(right))
        }
    }

    pub fn new_unary(operator: ExpressionType, right: Expression) -> Self {
        Self {
            expression_type: operator,
            value1: None,
            value2: Some(Box::new(right))
        }
    }
    
    pub fn new_empty(operator: ExpressionType) -> Self {
        Self {
            expression_type: operator,
            value1: None,
            value2: None
        }
    }
}

pub trait Executable: std::fmt::Debug {
    fn execute<'a>(&'a self, variables: &mut HashMap<&'a String, i64>) -> Result<(), String>;
}

#[derive(Debug)]
pub struct PrintStatement {
    expression: Option<Expression>,
    line: u64
}

impl PrintStatement {
    pub fn new(e: Option<Expression>, l: u64) -> Self {
        Self {
            expression: e,
            line: l
        }
    }
}

impl Executable for PrintStatement {
    fn execute(&self, variables: &mut HashMap<&String, i64>) -> Result<(), String> {
        if let Some(e) = &self.expression {
            print!("{} ", e.evaluate(variables).or_else(|s| Err(format!("{} at line {}", s, self.line)))?);
        }
        else {
            print!(" ");
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct PrintlnStatement {
    expression: Option<Expression>,
    line: u64
}

impl PrintlnStatement {
    pub fn new(e: Option<Expression>, l: u64) -> Self {
        Self {
            expression: e,
            line: l
        }
    }
}

impl Executable for PrintlnStatement {
    fn execute(&self, variables: &mut HashMap<&String, i64>) -> Result<(), String> {
        if let Some(e) = &self.expression {
            println!("{}", e.evaluate(variables).or_else(|s| Err(format!("{} at line {}", s, self.line)))?);
        }
        else {
            println!();
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct DefineStatement {
    variable_name: String,
    expression: Expression,
    line: u64
}

impl DefineStatement {
    pub fn new(v: String, e: Expression, l: u64) -> Self {
        Self {
            variable_name: v,
            expression: e,
            line: l
        }
    }
}

impl Executable for DefineStatement {
    fn execute<'a>(&'a self, variables: &mut HashMap<&'a String, i64>) -> Result<(), String> {
        if let Some(_) = variables.get(&self.variable_name) {
            return Err(format!("'{}' Redefining variable error at line {}", &self.variable_name, self.line));
        }
        let v = self.expression.evaluate(variables).or_else(|s| Err(format!("{} at line {}", s, self.line)))?;
        variables.insert(&self.variable_name, v);
        Ok(())
    }
}

#[derive(Debug)]
pub struct AssignStatement {
    variable_name: String,
    expression: Expression,
    line: u64
}

impl AssignStatement {
    pub fn new(v: String, e: Expression, l: u64) -> Self {
        Self {
            variable_name: v,
            expression: e,
            line: l
        }
    }
}

impl Executable for AssignStatement {
    fn execute<'a >(&'a self, variables: &mut HashMap<&'a String, i64>) -> Result<(), String> {
        if let None = variables.get(&self.variable_name) {
            return Err(format!("'{}' Undefined variable error at line {}", &self.variable_name, self.line));
        }
        let v = self.expression.evaluate(variables).or_else(|s| Err(format!("{} at line {}", s, self.line)))?;
        variables.insert(&self.variable_name, v);
        Ok(())
    }
}

#[derive(Debug)]
pub struct IfStatement {
    condition: Expression,
    statements: Vec<Box<dyn Executable>>,
    else_statements: Vec<Box<dyn Executable>>,
    line: u64
}

impl IfStatement {
    pub fn new(c: Expression, i: Vec<Box<dyn Executable>>, e: Vec<Box<dyn Executable>>, l: u64) -> Self {
        Self {
            condition: c,
            statements: i,
            else_statements: e,
            line: l
        }
    }
}

impl Executable for IfStatement {
    fn execute<'a>(&'a self, variables: &mut HashMap<&'a String, i64>) -> Result<(), String> {
        if self.condition.evaluate(variables).or_else(|s| Err(format!("{} at line {}", s, self.line)))? != 0 {
            for s in &self.statements {
                let _ = s.execute(variables)?;
            }
        }
        else {
            for s in &self.else_statements {
                let _ = s.execute(variables)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct WhileStatement {
    condition: Expression,
    statements: Vec<Box<dyn Executable>>,
    line: u64
}

impl WhileStatement {
    pub fn new(c: Expression, s: Vec<Box<dyn Executable>>, l: u64) -> Self {
        Self {
            condition: c,
            statements: s,
            line: l
        }
    }
}

impl Executable for WhileStatement {
    fn execute<'a>(&'a self, variables: &mut HashMap<&'a String, i64>) -> Result<(), String> {
        while self.condition.evaluate(variables).or_else(|s| Err(format!("{} at line {}", s, self.line)))? != 0 {
            for s in &self.statements {
                let _ = s.execute(variables)?;
            }
        }
        Ok(())    
    }
}
