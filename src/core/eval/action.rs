use crate::core::ast::Stmt;
use crate::core::eval::value::{Context, Value};
use anyhow::{Result, bail};

/// Pure action output from statement execution
#[derive(Debug, Clone)]
pub enum Action {
    /// Continue to next line
    Continue,
    /// Jump to line number
    Jump(u32),
    /// Output value
    Output(String),
    /// Request input
    Input(String),
    /// Assign value to variable
    Assign(String, Value),
    /// End execution
    End,
}

impl Stmt {
    pub fn execute(&self, ctx: &Context) -> Result<Action> {
        match self {
            Stmt::Rem { .. } => Ok(Action::Continue),

            Stmt::Goto { lineno } => Ok(Action::Jump(*lineno)),

            Stmt::Print { expr } => {
                let val = expr.evaluate(ctx)?;
                Ok(Action::Output(val.to_string()))
            }

            Stmt::Input { name } => Ok(Action::Input(name.clone())),

            Stmt::Let { name, expr } => {
                let val = expr.evaluate(ctx)?;
                Ok(Action::Assign(name.clone(), val))
            }

            Stmt::IfThen {
                conditional,
                lineno,
            } => {
                let val = conditional.evaluate(ctx)?;
                let is_true = match val {
                    Value::Boolean(b) => b,
                    _ => bail!("Type mismatch in IF condition"),
                };

                if is_true {
                    Ok(Action::Jump(*lineno))
                } else {
                    Ok(Action::Continue)
                }
            }

            Stmt::End => Ok(Action::End),
        }
    }
}
