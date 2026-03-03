/// Events that the interpreter can emit for UI consumption
#[derive(Debug, Clone, PartialEq)]
pub enum InterpreterEvent {
    /// Output text to display
    Output(String),
    /// Request input for a variable
    Input(String),
    /// Program has finished execution
    Finished,
    /// Runtime error occurred
    Error(String),
    /// Debug information (line number changes, etc.)
    Debug(String),
}

/// Simple event queue for decoupled UI communication
#[derive(Debug, Default)]
pub struct EventQueue {
    events: Vec<InterpreterEvent>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Push an event to the queue
    pub fn push(&mut self, event: InterpreterEvent) {
        self.events.push(event);
    }

    /// Take all events from the queue
    pub fn take_events(&mut self) -> Vec<InterpreterEvent> {
        std::mem::take(&mut self.events)
    }

    /// Check if there are any events
    pub fn has_events(&self) -> bool {
        !self.events.is_empty()
    }

    /// Get the number of pending events
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
