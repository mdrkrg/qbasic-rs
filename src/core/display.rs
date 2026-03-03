use std::fmt::Display;

use crate::core::{
    ast::{Expr, Line, Stmt},
    eval::Interpreter,
};

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary {
                operator,
                left,
                right,
            } => write!(f, "    {operator}\n    {left}\n    {right}"),
            Expr::Unary { operator, right } => write!(f, "    {operator}\n{right}"),
            Expr::Grouping { expression } => write!(f, "    (\n{expression}    )"),
            Expr::Literal(literal_value) => write!(f, "    {literal_value}"),
            Expr::Variable { name } => write!(f, "    {name}"),
        }
    }
}

impl Line {
    /// Format the syntax tree using the context of an interpreter
    pub fn format_syntax_tree(self: &Line, interpreter: &Interpreter) -> String {
        let lineno = &self.lineno;
        let rest = match &self.statement {
            Stmt::Let { name, expr } => self.format_let(interpreter, name, expr),
            Stmt::Rem { comment } => self.format_rem(interpreter, comment),
            Stmt::Print { expr } => self.format_print(interpreter, expr),
            Stmt::Input { name } => self.format_input(interpreter, name),
            Stmt::Goto { lineno } => self.format_goto(interpreter, lineno),
            Stmt::IfThen {
                conditional,
                lineno,
            } => self.format_if_then(interpreter, conditional, lineno),
            Stmt::End => self.format_end(interpreter),
        };
        format!("{lineno} {rest}")
    }

    /// Format a line of LET statement
    /// Special: Track variable use count
    fn format_let(self: &Line, interpreter: &Interpreter, name: &str, expr: &Expr) -> String {
        let count = self.get_execution_count(interpreter);

        let usage_count = interpreter.context().variable_use_counts.borrow();
        let usage_count = usage_count.get(name).unwrap_or(&0);

        let first_line = format!("LET = {count}");
        let second_line = format!("    {name} {usage_count}");

        format!("{first_line}\n{second_line}\n{expr}")
    }

    /// Format a line of REM statement
    fn format_rem(self: &Line, interpreter: &Interpreter, comment: &str) -> String {
        let count = self.get_execution_count(interpreter);
        format!("REM {count}\n    {comment}")
    }

    /// Format a line of PRINT statement
    fn format_print(self: &Line, interpreter: &Interpreter, expr: &Expr) -> String {
        let count = self.get_execution_count(interpreter);
        format!("PRINT {count}\n{expr}")
    }

    /// Format a line of INPUT statement
    fn format_input(self: &Line, interpreter: &Interpreter, name: &str) -> String {
        let count = self.get_execution_count(interpreter);
        format!("INPUT {count}\n    {name}")
    }

    /// Format a line of GOTO statement
    fn format_goto(self: &Line, interpreter: &Interpreter, lineno: &u32) -> String {
        let count = self.get_execution_count(interpreter);
        format!("GOTO {count}\n    {lineno}")
    }

    /// Format a line of IF THEN statement
    /// Special: Track branch taken and not taken count
    fn format_if_then(
        self: &Line,
        interpreter: &Interpreter,
        conditional: &Expr,
        lineno: &u32,
    ) -> String {
        let (true_count, false_count) = interpreter
            .line_stats()
            .get(&self.lineno)
            .map_or_else(|| (0, 0), |stat| (stat.if_true_count, stat.if_false_count));

        let first_line = format!("IF THEN {true_count} {false_count}");
        format!("{first_line}\n{conditional}\n    {lineno}")
    }

    /// Format a line of END statement
    fn format_end(self: &Line, interpreter: &Interpreter) -> String {
        format!("END {}", self.get_execution_count(interpreter))
    }

    /// Utility to get execution count of a line, using the context of an interpreter
    fn get_execution_count(self: &Line, interpreter: &Interpreter) -> u32 {
        interpreter
            .line_stats()
            .get(&self.lineno)
            .map_or_else(|| 0, |stat| stat.execution_count)
    }
}

impl Interpreter {
    /// Generate syntax tree for a line, using the context of an interpreter
    pub fn generate_syntax_tree(&self, line: &Line) -> String {
        line.format_syntax_tree(self)
    }
}
