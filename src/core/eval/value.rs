use crate::core::{
    ast::{BinaryOp, Expr, LiteralValue, UnaryOp},
    token::{Math, Relational},
};
use anyhow::{Result, bail};
use std::str::FromStr;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    String(String),
    Boolean(bool),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{i}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::None => writeln!(f),
        }
    }
}

impl TryInto<i64> for Value {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<i64, Self::Error> {
        match self {
            Value::Integer(i) => Ok(i),
            _ => bail!("Type Mismatch: Expected Integer"),
        }
    }
}

impl TryInto<String> for Value {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<String, Self::Error> {
        match self {
            Value::String(s) => Ok(s),
            _ => bail!("Type mismatch: Expected String"),
        }
    }
}

impl TryInto<bool> for Value {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<bool, Self::Error> {
        match self {
            Value::Boolean(b) => Ok(b),
            _ => bail!("Type mismatch: Expected Boolean"),
        }
    }
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let clean_input = s.trim();

        // Try integer first
        if let Ok(i) = clean_input.parse::<i64>() {
            return Ok(Value::Integer(i));
        }

        // Fallback to string
        Ok(Value::String(clean_input.to_string()))
    }
}

#[derive(Default, Clone, Debug)]
pub struct Context {
    pub variables: HashMap<String, Value>,
}

impl Expr {
    pub fn evaluate(&self, context: &Context) -> Result<Value> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                Expr::eval_binary(left_val, *operator, right_val)
            }

            Expr::Unary { operator, right } => {
                let right_val = right.evaluate(context)?;
                Expr::eval_unary(*operator, right_val)
            }

            Expr::Grouping { expression } => expression.evaluate(context),

            Expr::Literal(literal) => Ok(Expr::eval_literal(literal)),

            Expr::Variable { name } => match context.variables.get(name) {
                Some(val) => Ok(val.clone()),
                None => Ok(Value::Integer(0)),
            },
        }
    }

    fn eval_literal(literal: &LiteralValue) -> Value {
        match literal {
            LiteralValue::Integer(i) => Value::Integer(*i as i64),
            LiteralValue::String(s) => Value::String(s.clone()),
            LiteralValue::None => Value::None,
            LiteralValue::Number(_) => todo!("Number is not supported yet"),
        }
    }

    fn eval_binary(left: Value, op: BinaryOp, right: Value) -> Result<Value> {
        let result = match op {
            BinaryOp::Arithmetic(math) => {
                let l: i64 = left.clone().try_into()?;
                let r: i64 = right.clone().try_into()?;
                let int = match math {
                    Math::Plus => l + r,
                    Math::Minus => l - r,
                    Math::Times => l * r,
                    Math::Division => {
                        if r == 0 {
                            bail!("Division by zero");
                        }
                        l / r
                    }
                    Math::Modulo => l % r,
                    Math::Power => (l as f64).powf(r as f64) as i64,
                };
                Value::Integer(int)
            }
            BinaryOp::Relational(relational) => match (left, right) {
                (Value::Integer(l), Value::Integer(r)) => {
                    let bool = match relational {
                        Relational::Eq => l.eq(&r),
                        Relational::NotEq => l.ne(&r),
                        Relational::Lt => l.lt(&r),
                        Relational::LtEq => l.le(&r),
                        Relational::Gt => l.gt(&r),
                        Relational::GtEq => l.ge(&r),
                    };
                    Value::Boolean(bool)
                }
                (Value::Integer(_), Value::String(_)) | (Value::String(_), Value::Integer(_)) => {
                    match relational {
                        Relational::Eq => Value::Boolean(false),
                        Relational::NotEq => Value::Boolean(true),
                        _ => bail!("Cannot compare integer to string"),
                    }
                }
                (Value::String(l), Value::String(r)) => {
                    let bool = match relational {
                        Relational::Eq => l.eq(&r),
                        Relational::NotEq => l.ne(&r),
                        Relational::Lt => l.lt(&r),
                        Relational::LtEq => l.le(&r),
                        Relational::Gt => l.gt(&r),
                        Relational::GtEq => l.ge(&r),
                    };
                    Value::Boolean(bool)
                }
                _ => {
                    bail!("Unexpected comparison")
                }
            },
        };

        Ok(result)
    }

    fn eval_unary(op: UnaryOp, right: Value) -> Result<Value> {
        let r: i64 = right.clone().try_into()?;
        match op {
            UnaryOp::Negate => Ok(Value::Integer(-r)),
        }
    }
}
