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
/// Wrapper of the Rust internal interpreter
pub struct Interpreter {
    /// The inner, "real" interpreter
    inner: RustInterpreter,
    /// Interpreter Line structs
    program: BTreeMap<u32, Line>,
    /// Source code text
    program_texts: BTreeMap<u32, String>,
    /// Formatted syntax tree to be displayed in the frontend
    syntax_trees: HashMap<u32, String>,
}

/// Construct a new Interpreter, return a pointer to Rust opaque type
fn new_interpreter() -> Box<Interpreter> {
    Box::new(Interpreter::default())
}
