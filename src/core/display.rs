use std::fmt::Display;

use crate::core::ast::{Expr, Line, Stmt};

/// Interface to provide line statistics
pub trait StatsProvider {
    /// Get execution count of a line
    fn execution_count(&self, lineno: u32) -> u32;

    /// Get branching statstics of an IF ELSE statement
    fn if_branch_counts(&self, lineno: u32) -> (u32, u32);

    /// Get usage count of a variable
    fn variable_use_count(&self, name: &str) -> u32;
}

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
    pub fn format_syntax_tree(self: &Line, stats: &dyn StatsProvider) -> String {
        let lineno = &self.lineno;
        let rest = match &self.statement {
            Stmt::Let { name, expr } => self.format_let(stats, name, expr),
            Stmt::Rem { comment } => self.format_rem(stats, comment),
            Stmt::Print { expr } => self.format_print(stats, expr),
            Stmt::Input { name } => self.format_input(stats, name),
            Stmt::Goto { lineno } => self.format_goto(stats, lineno),
            Stmt::IfThen {
                conditional,
                lineno,
            } => self.format_if_then(stats, conditional, lineno),
            Stmt::End => self.format_end(stats),
        };
        format!("{lineno} {rest}")
    }

    /// Format a line of LET statement
    /// Special: Track variable use count
    fn format_let(self: &Line, stats: &dyn StatsProvider, name: &str, expr: &Expr) -> String {
        let count = stats.execution_count(self.lineno);
        let usage_count = stats.variable_use_count(name);

        let first_line = format!("LET = {count}");
        let second_line = format!("    {name} {usage_count}");

        format!("{first_line}\n{second_line}\n{expr}")
    }

    /// Format a line of REM statement
    fn format_rem(self: &Line, stats: &dyn StatsProvider, comment: &str) -> String {
        let count = stats.execution_count(self.lineno);
        format!("REM {count}\n    {comment}")
    }

    /// Format a line of PRINT statement
    fn format_print(self: &Line, stats: &dyn StatsProvider, expr: &Expr) -> String {
        let count = stats.execution_count(self.lineno);
        format!("PRINT {count}\n{expr}")
    }

    /// Format a line of INPUT statement
    fn format_input(self: &Line, stats: &dyn StatsProvider, name: &str) -> String {
        let count = stats.execution_count(self.lineno);
        format!("INPUT {count}\n    {name}")
    }

    /// Format a line of GOTO statement
    fn format_goto(self: &Line, stats: &dyn StatsProvider, lineno: &u32) -> String {
        let count = stats.execution_count(self.lineno);
        format!("GOTO {count}\n    {lineno}")
    }

    /// Format a line of IF THEN statement
    /// Special: Track branch taken and not taken count
    fn format_if_then(
        self: &Line,
        stats: &dyn StatsProvider,
        conditional: &Expr,
        lineno: &u32,
    ) -> String {
        let (true_count, false_count) = stats.if_branch_counts(self.lineno);
        let first_line = format!("IF THEN {true_count} {false_count}");
        format!("{first_line}\n{conditional}\n    {lineno}")
    }

    /// Format a line of END statement
    fn format_end(self: &Line, stats: &dyn StatsProvider) -> String {
        let count = stats.execution_count(self.lineno);
        format!("END {count}")
    }
}
