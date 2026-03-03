use crate::core::ast::{Line, Stmt};
use crate::core::eval::{
    action::Action,
    event::{EventQueue, InterpreterEvent},
    value::{Context, Value},
};
use anyhow::{Result, bail};
use getset::Getters;
use std::collections::{BTreeMap, HashMap};

/// Statistics for a single line
#[derive(Debug, Clone, Default)]
pub struct LineStats {
    /// Number of times the line has been executed
    pub execution_count: u32,
    /// Number of times IF branch taken
    pub if_true_count: u32,
    /// Number of times IF branch not taken
    pub if_false_count: u32,
}

/// State machine that manages program execution
#[derive(Debug, Getters, Default)]
pub struct Interpreter {
    #[getset(get = "pub")]
    context: Context,
    #[getset(get = "pub")]
    program: BTreeMap<u32, Stmt>,
    #[getset(get = "pub")]
    pc: u32,
    #[getset(get = "pub")]
    state: InterpreterState,
    events: EventQueue,

    /// Statistics for each line
    #[getset(get = "pub")]
    line_stats: HashMap<u32, LineStats>,
    /// Count variable usage count
    /// TODO: implement this
    #[getset(get = "pub")]
    variable_use_counts: HashMap<String, u32>,
}

/// Current state of the interpreter
#[derive(Debug, Clone, PartialEq, Default)]
pub enum InterpreterState {
    /// Ready to execute next statement
    #[default]
    Ready,
    /// Waiting for input for specified variable
    WaitingForInput(String),
    /// Program has finished execution
    Finished,
    /// Runtime error occurred
    Error(String),
}

impl Interpreter {
    /// Create a new interpreter from program lines
    pub fn new(lines: Vec<Line>) -> Self {
        let mut program = BTreeMap::new();
        let pc = lines.first().map(|line| line.lineno).unwrap_or(0);

        for line in lines {
            program.insert(line.lineno, line.statement);
        }

        Self {
            context: Context::default(),
            program,
            pc,
            state: InterpreterState::Ready,
            events: EventQueue::new(),
            line_stats: HashMap::new(),
            variable_use_counts: HashMap::new(),
        }
    }

    /// Take all pending events from the queue
    pub fn take_events(&mut self) -> Vec<InterpreterEvent> {
        self.events.take_events()
    }

    /// Check if there are pending events
    pub fn has_events(&self) -> bool {
        self.events.has_events()
    }

    /// Go to next line after output
    pub fn next(&mut self) {
        self.handle_action(Action::Continue);
    }

    /// Reset all statistics
    pub fn reset_statistics(&mut self) {
        self.line_stats.clear();
        self.variable_use_counts.clear();
    }

    /// Execute one step of the program
    pub fn step(&mut self) {
        if self.state != InterpreterState::Ready {
            // Not ready, cannot step
            return;
        }

        match self.program.get(&self.pc) {
            Some(stmt) => {
                // Track execution count
                let stats = self.line_stats.entry(self.pc).or_default();
                stats.execution_count += 1;

                match stmt.execute(&self.context) {
                    Ok(action) => {
                        // Track branch results
                        if let Stmt::IfThen { .. } = stmt {
                            let stats = self.line_stats.entry(self.pc).or_default();
                            match action {
                                Action::Jump(_) => stats.if_true_count += 1,
                                Action::Continue => stats.if_false_count += 1,
                                _ => {} // Should not happen
                            }
                        }
                        self.handle_action(action)
                    }
                    Err(err) => {
                        let error_msg = err.to_string();
                        self.state = InterpreterState::Error(error_msg.clone());
                        self.events.push(InterpreterEvent::Error(error_msg));
                    }
                }
            }
            None => {
                self.state = InterpreterState::Finished;
                self.events.push(InterpreterEvent::Finished);
            }
        }
    }

    /// Handle a pure action from statement execution
    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Continue => {
                // Move to next line
                if let Some(&next_line) = self.program.keys().find(|&&k| k > self.pc) {
                    self.pc = next_line;
                    // Debug: line change
                    self.events.push(InterpreterEvent::Debug(format!(
                        "Moving to line {}",
                        self.pc
                    )));
                } else {
                    // Cannot find next line, we are done
                    // WARN: Distinguish between END and not terminated
                    self.state = InterpreterState::Finished;
                    self.events.push(InterpreterEvent::Finished);
                }
            }
            Action::Jump(lineno) => {
                if self.program.contains_key(&lineno) {
                    self.pc = lineno;
                    self.events.push(InterpreterEvent::Debug(format!(
                        "Jumping to line {}",
                        self.pc
                    )));
                } else {
                    let error = format!("Line {} not found", lineno);
                    self.state = InterpreterState::Error(error.clone());
                    self.events.push(InterpreterEvent::Error(error));
                }
            }
            Action::Output(text) => {
                self.events.push(InterpreterEvent::Output(text));
                // UI should resume manually calling step() after output
            }
            Action::Input(name) => {
                self.state = InterpreterState::WaitingForInput(name.clone());
                self.events.push(InterpreterEvent::Input(name));
            }
            Action::Assign(name, value) => {
                self.context.variables.insert(name, value);
                self.handle_action(Action::Continue);
            }
            Action::End => {
                self.state = InterpreterState::Finished;
                self.events.push(InterpreterEvent::Finished);
            }
        }
    }

    /// Provide input value when interpreter is waiting for input
    pub fn provide_input(&mut self, value: Value) -> Result<()> {
        let var_name = match &self.state {
            InterpreterState::WaitingForInput(name) => name.clone(),
            _ => bail!("Interpreter is not waiting for input"),
        };

        self.context
            .variables
            .insert(var_name.clone(), value.clone());
        self.state = InterpreterState::Ready;

        // Emit debug event for input received
        self.events.push(InterpreterEvent::Debug(format!(
            "Input received for {}: {}",
            var_name, value
        )));

        // Move to next line
        if let Some(&next_line) = self.program.keys().find(|&&k| k > self.pc) {
            self.pc = next_line;
            self.events.push(InterpreterEvent::Debug(format!(
                "Moving to line {} after input",
                self.pc
            )));
        } else {
            self.state = InterpreterState::Finished;
            self.events.push(InterpreterEvent::Finished);
        }
        Ok(())
    }
}

impl Interpreter {
    /// Run program to completion (for testing/debugging)
    #[cfg(feature = "testing")]
    pub fn run_test(&mut self) {
        while self.state == InterpreterState::Ready {
            self.step();
            // Auto continue when output
            let events = self.take_events();
            let mut should_continue = false;

            for event in events {
                match event {
                    InterpreterEvent::Output(text) => {
                        println!("{}", text);
                        should_continue = true;
                    }
                    InterpreterEvent::Input(_) => {
                        // Skip input when testing
                        return;
                    }
                    _ => (),
                }
            }

            if should_continue {
                // Auto continue after output for testing
                self.handle_action(Action::Continue);
            }
        }
    }

    pub fn run_bin(&mut self) {
        while self.state == InterpreterState::Ready {
            self.step();
            let events = self.take_events();
            let mut should_continue = false;

            for event in events {
                match event {
                    InterpreterEvent::Output(text) => {
                        println!("{}", text);
                        should_continue = true;
                    }
                    InterpreterEvent::Input(_) => {
                        use std::str::FromStr;

                        let mut buf = String::new();
                        let stdin = std::io::stdin();
                        if let Err(err) = stdin.read_line(&mut buf) {
                            eprintln!("Error: {err}");
                            return;
                        };
                        let _ = match Value::from_str(&buf) {
                            Ok(value) => self.provide_input(value),
                            Err(err) => {
                                eprintln!("Error: {err}");
                                return;
                            }
                        };
                    }
                    _ => {
                        // Ignore other events in run mode
                    }
                }
            }

            if should_continue {
                self.handle_action(Action::Continue);
            }
        }
    }
}
