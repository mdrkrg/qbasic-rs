use crate::core::{ast::Line, eval::interpreter::Interpreter as RustInterpreter};
use std::collections::{BTreeMap, HashMap};

mod implement;

#[cxx::bridge(namespace = "qbasic_rs")]
mod qbasic_rs {
    // CXX representation of Rust Line and syntax tree
    #[derive(Debug, Clone)]
    struct ProgramLine {
        lineno: u32,
        text: String,
        syntax_tree: String,
    }

    /// CXX representation of variable name value mapping, and use count
    #[derive(Debug, Clone)]
    struct Variable {
        name: String,
        value: String,
        use_count: u32,
    }

    /// CXX representation of lineno and Rust LineStats
    #[derive(Debug, Clone)]
    struct LineStats {
        lineno: u32,
        execution_count: u32,
        if_true_count: u32,
        if_false_count: u32,
    }

    /// Plain CXX enum of Rust InterpreterState
    #[derive(Debug, Clone, PartialEq)]
    enum InterpreterState {
        Ready,
        WaitingForInput, // Get name by get_waiting_for_input()
        Finished,        // Get message by get_error_message()
        Error,
    }

    /// A batch of interpreter events, as the result and side effect of execution
    #[derive(Debug, Clone)]
    struct EventBatch {
        /// Output contents
        outputs: Vec<String>,
        /// Variable names to input
        inputs: Vec<String>,
        /// Error messages
        errors: Vec<String>,
        /// Debug messages
        debug_messages: Vec<String>,
        /// Whether interpreter has finished
        finished: bool,
    }

    /// Line operation result, for UI update in sync
    #[derive(Debug, Clone)]
    struct LineOpResult {
        op_type: LineOpType,
        lineno: u32,
    }

    /// Line operation: added / updated or deleted, for UI update in sync
    #[derive(Debug, Clone)]
    enum LineOpType {
        Added,
        Deleted,
    }

    extern "Rust" {
        type Interpreter;

        fn new_interpreter() -> Box<Interpreter>;

        fn process_line(self: &mut Interpreter, line_text: &str) -> Result<LineOpResult>;
        fn clear(self: &mut Interpreter);

        fn load_file(self: &mut Interpreter, path: &str) -> Result<()>;
        fn get_program_lines(self: &Interpreter) -> Vec<ProgramLine>;

        fn run(self: &mut Interpreter) -> EventBatch;
        fn step(self: &mut Interpreter) -> EventBatch;
        fn reset(self: &mut Interpreter);

        fn provide_input(self: &mut Interpreter, value: &str) -> Result<()>;

        fn get_state(self: &Interpreter) -> InterpreterState;
        fn can_edit(self: &Interpreter) -> bool;
        fn get_current_line(self: &Interpreter) -> u32;
        fn get_error_message(self: &Interpreter) -> String;
        fn get_waiting_for_input(self: &Interpreter) -> String;

        fn get_variables(self: &Interpreter) -> Vec<Variable>;
        fn get_line_stats(self: &Interpreter) -> Vec<LineStats>;
        fn get_syntax_tree(self: &Interpreter, lineno: u32) -> Result<String>;
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
    /// Whether execution has started (for can_edit)
    started: bool,
}

/// Construct a new Interpreter
fn new_interpreter() -> Box<Interpreter> {
    Box::new(Interpreter::default())
}
