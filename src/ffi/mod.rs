use crate::core::{ast::Line, eval::interpreter::Interpreter as RustInterpreter};
use std::collections::{BTreeMap, HashMap};

#[cxx::bridge(namespace = "qbasic_rs")]
mod qbasic_rs {
    extern "Rust" {
        type Interpreter;

        fn new_interpreter() -> Box<Interpreter>;
    }
}

#[derive(Default)]
/// Wrapper of the Rust internal interpreter that provide some statistics of
/// program execution
pub struct Interpreter {
    /// The inner, "real" interpreter
    inner: RustInterpreter,
    /// Interpreter Line structs
    program: BTreeMap<u32, Line>,
    /// Source code text
    program_texts: BTreeMap<u32, String>,
    /// Formatted syntax tree to be displayed in the frontend
    syntax_trees: HashMap<u32, String>,
    /// Statistics of line execution
    line_stats: HashMap<u32, LineStatsInternal>,
    /// Statistics of variable usage
    variable_use_counts: HashMap<String, u32>,
}

struct LineStatsInternal {
    /// Number of times the line of program has been executed
    execution_count: u32,
    /// Number of times a "true" branch is been taken of a IF statement
    if_true_count: u32,
    /// Number of times a "false" branch is been taken of a IF statement
    if_false_count: u32,
}

/// Construct a new Interpreter, return a pointer to Rust opaque type
fn new_interpreter() -> Box<Interpreter> {
    Box::new(Interpreter::default())
}
