use crate::core::{
    ast::{Line, Stmt},
    eval::{
        event::InterpreterEvent as RustEvent,
        interpreter::{Interpreter as RustInterpreter, InterpreterState as RustState},
        value::Value,
    },
    lexer::tokenize,
    parser::Parser,
};
use anyhow::{Context as AnyhowContext, Result, bail};
use std::str::FromStr;

use super::Interpreter;
use super::qbasic_rs::{
    EventBatch, InterpreterState, LineOpResult, LineOpType, LineStats, ProgramLine, Variable,
};

impl Interpreter {
    /// Check if program can be edited
    ///
    /// Editing is allowed when:
    /// - Interpreter is Ready and execution has not started
    /// - Interpreter is Finished
    /// - Interpreter is Error
    ///
    /// Editing is not allowed when:
    /// - Interpreter is WaitingForInput
    /// - Interpreter is Ready AND execution has started
    pub fn can_edit(&self) -> bool {
        match self.get_state() {
            InterpreterState::Finished | InterpreterState::Error => true,
            InterpreterState::Ready => !self.started,
            InterpreterState::WaitingForInput => false,
            _ => false,
        }
    }

    /// Execute a statement directly (without line number)
    /// Allowed: LET, PRINT, INPUT
    pub fn execute(&mut self, line_text: &str) -> Result<EventBatch> {
        let tokens =
            tokenize(line_text).map_err(|e| anyhow::anyhow!("Tokenization error: {:?}", e))?;
        let mut parser = Parser::new(tokens);
        let stmt = parser
            .statement()
            .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

        match &stmt {
            Stmt::Let { .. } | Stmt::Print { .. } | Stmt::Input { .. } => {
                // Execute directly
                self.inner.execute(stmt);
                let events = self.inner.take_events();
                let mut batch = EventBatch {
                    outputs: Vec::new(),
                    inputs: Vec::new(),
                    errors: Vec::new(),
                    debug_messages: Vec::new(),
                    finished: false,
                };
                for event in events {
                    match event {
                        RustEvent::Output(text) => batch.outputs.push(text),
                        RustEvent::Input(var_name) => batch.inputs.push(var_name),
                        RustEvent::Error(err) => batch.errors.push(err),
                        RustEvent::Finished => batch.finished = true,
                        RustEvent::Debug(msg) => batch.debug_messages.push(msg),
                    }
                }
                Ok(batch)
            }
            _ => bail!("Statement not allowed in direct mode: {:?}", stmt),
        }
    }

    /// Process a line edit command
    /// - If line is a number, delete the line (ignore if not exist)
    /// - Otherwise, add / update the line
    pub fn process_line(&mut self, line_text: &str) -> Result<LineOpResult> {
        if !self.can_edit() {
            bail!("Cannot edit program while interpreter is running")
        }

        // Check if only line number
        {
            let trimmed = line_text.trim();
            if let Ok(lineno) = trimmed.parse::<u32>() {
                // Only line number, delete line
                self.delete_line(lineno)?;
                return Ok(LineOpResult {
                    op_type: LineOpType::Deleted,
                    lineno,
                });
            }
        }

        // Add / update
        {
            let lineno = self.add_line(line_text)?;
            Ok(LineOpResult {
                op_type: LineOpType::Added,
                lineno,
            })
        }
    }

    /// Parse a line of source code
    fn parse_line(&self, line_text: &str) -> Result<Line> {
        let tokens =
            tokenize(line_text).map_err(|e| anyhow::anyhow!("Tokenization error: {:?}", e))?;
        let mut parser = Parser::new(tokens);
        parser
            .line()
            .map_err(|e| anyhow::anyhow!("Parse error: {}", e))
    }

    /// Generate a syntax tree string for a line
    fn generate_syntax_tree(&self, line: &Line) -> String {
        line.format_syntax_tree(&self.inner)
    }

    /// Rebuild the interpreter, reset state
    fn rebuild(&mut self) {
        let mut lines: Vec<Line> = self.program.values().cloned().collect();
        lines.sort_by_key(|l| l.lineno);
        self.inner = RustInterpreter::new(lines);
        self.started = false;
    }

    /// Add a line of code to the interpreter state
    fn add_line(&mut self, line_text: &str) -> Result<u32> {
        let line = self.parse_line(line_text).context("Failed to parse line")?;
        let lineno = line.lineno;

        // Store Line and text
        {
            self.program.insert(lineno, line.clone());
            self.program_texts.insert(lineno, line_text.to_string());
        }

        // Generate syntax tree
        {
            self.syntax_trees
                .insert(lineno, self.generate_syntax_tree(&line));
        }

        {
            // WARN: context lost?
            self.rebuild();
        }

        Ok(lineno)
    }

    /// Delete a line of code from the interpreter state
    fn delete_line(&mut self, lineno: u32) -> Result<()> {
        if self.program.remove(&lineno).is_none() {
            bail!("Line {} not found", lineno)
        }

        {
            self.program_texts.remove(&lineno);
            self.syntax_trees.remove(&lineno);
        }

        {
            // WARN: context lost?
            self.rebuild();
        }

        Ok(())
    }

    /// Clear all states and reset the interpreter
    pub fn clear(&mut self) {
        self.inner = RustInterpreter::new(Vec::new());
        self.program.clear();
        self.program_texts.clear();
        self.syntax_trees.clear();
        self.started = false;
    }

    /// Load a file as source code and rebuild the interpreter
    pub fn load_file(&mut self, path: &str) -> Result<()> {
        use std::fs;

        let content = fs::read_to_string(path).context(format!("Failed to read file: {}", path))?;

        self.clear();

        // for each line, do parse
        for line_str in content.lines() {
            if line_str.trim().is_empty() {
                continue;
            }

            let line = self.parse_line(line_str)?;
            let lineno = line.lineno;

            self.program.insert(lineno, line.clone());
            self.program_texts.insert(lineno, line_str.to_string());
            self.syntax_trees
                .insert(lineno, self.generate_syntax_tree(&line));
        }

        // WARN: context lost?
        self.rebuild();
        Ok(())
    }

    /// Get program lines with syntax trees
    pub fn get_program_lines(&self) -> Vec<ProgramLine> {
        let mut lines = Vec::new();

        for (&lineno, text) in &self.program_texts {
            let syntax_tree = self
                .syntax_trees
                .get(&lineno)
                .cloned()
                .unwrap_or_else(|| "No syntax tree".to_string());

            lines.push(ProgramLine {
                lineno,
                text: text.clone(),
                syntax_tree,
            });
        }

        // Sort by line number
        lines.sort_by_key(|l| l.lineno);
        lines
    }

    /// Execute one interpreter step and return the side effects
    /// that this step will generate
    pub fn step(&mut self) -> EventBatch {
        self.started = true;
        let mut batch = EventBatch {
            outputs: Vec::new(),
            inputs: Vec::new(),
            errors: Vec::new(),
            debug_messages: Vec::new(),
            finished: false,
        };

        self.inner.step();

        // Collect events
        let rust_events = self.inner.take_events();
        for event in rust_events {
            match event {
                RustEvent::Output(text) => {
                    batch.outputs.push(text);
                }
                RustEvent::Input(var_name) => {
                    batch.inputs.push(var_name);
                }
                RustEvent::Error(err) => {
                    batch.errors.push(err);
                }
                RustEvent::Finished => {
                    batch.finished = true;
                }
                RustEvent::Debug(msg) => {
                    batch.debug_messages.push(msg);
                }
            }
        }

        batch
    }

    /// Run the interpreter until interrupt (input event), error or finish
    pub fn run(&mut self) -> EventBatch {
        self.started = true;
        let mut batch = EventBatch {
            outputs: Vec::new(),
            inputs: Vec::new(),
            errors: Vec::new(),
            debug_messages: Vec::new(),
            finished: false,
        };

        // Run until not ready
        while self.get_state() == InterpreterState::Ready {
            let step_batch = self.step();

            // Merge event batch
            {
                batch.outputs.extend(step_batch.outputs);
                batch.inputs.extend(step_batch.inputs);
                batch.errors.extend(step_batch.errors);
                batch.debug_messages.extend(step_batch.debug_messages);
                batch.finished = batch.finished || step_batch.finished;
            }

            // If waiting for input, stop execution
            if self.get_state() == InterpreterState::WaitingForInput {
                break;
            }
        }

        batch
    }

    /// Reset interpreter state but keep program
    pub fn reset(&mut self) {
        let mut lines: Vec<Line> = self.program.values().cloned().collect();
        lines.sort_by_key(|l| l.lineno);
        self.inner = RustInterpreter::new(lines);
        self.started = false;
    }

    /// Provide input to the interpreter state if it is requiring it
    pub fn provide_input(&mut self, value: &str) -> Result<()> {
        let val = Value::from_str(value).context(format!("Failed to parse value '{}'", value))?;

        self.inner
            .provide_input(val)
            .context("Failed to provide input")
    }

    /// Get interpreter state
    pub fn get_state(&self) -> InterpreterState {
        match self.inner.state() {
            RustState::Ready => InterpreterState::Ready,
            RustState::WaitingForInput(_) => InterpreterState::WaitingForInput,
            RustState::Finished => InterpreterState::Finished,
            RustState::Error(_) => InterpreterState::Error,
        }
    }

    /// Get current line number
    pub fn get_current_line(&self) -> u32 {
        *self.inner.pc()
    }

    /// Get the error message if the interpreter is in error state
    pub fn get_error_message(&self) -> String {
        match self.inner.state() {
            RustState::Error(msg) => msg.clone(),
            _ => String::new(),
        }
    }

    /// Get the name of the variable if the program is waiting for input
    pub fn get_waiting_for_input(&self) -> String {
        match self.inner.state() {
            RustState::WaitingForInput(var_name) => var_name.clone(),
            _ => String::new(),
        }
    }

    /// Get variables and usage statistics
    pub fn get_variables(&self) -> Vec<Variable> {
        let context = self.inner.context();
        let use_counts = context.variable_use_counts.borrow();
        let mut vars = Vec::new();

        for (name, value) in &context.variables {
            let use_count = use_counts.get(name).copied().unwrap_or(0);

            vars.push(Variable {
                name: name.clone(),
                value: value.to_string(),
                use_count,
            });
        }

        vars
    }

    /// Get line execution statistics
    pub fn get_line_stats(&self) -> Vec<LineStats> {
        let rust_stats = self.inner.line_stats();
        rust_stats
            .iter()
            .map(|(&lineno, stats)| LineStats {
                lineno,
                execution_count: stats.execution_count,
                if_true_count: stats.if_true_count,
                if_false_count: stats.if_false_count,
            })
            .collect()
    }

    /// Get syntax tree at line number
    pub fn get_syntax_tree(&self, lineno: u32) -> Result<String> {
        self.syntax_trees
            .get(&lineno)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Line {} not found", lineno))
    }
}
