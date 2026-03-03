use qbasic_rs::ffi::{Interpreter, InterpreterState};

#[test]
fn test_edit_before_execution() {
    let mut interpreter = Interpreter::default();

    // Should be able to edit before execution starts
    assert!(interpreter.process_line("10 PRINT 1").is_ok());
    assert!(interpreter.process_line("20 LET x = 5").is_ok());

    // Check program lines
    let lines = interpreter.get_program_lines();
    assert_eq!(lines.len(), 2);
}

#[test]
fn test_edit_during_execution() {
    let mut interpreter = Interpreter::default();

    // Add some lines
    interpreter.process_line("10 PRINT 1").unwrap();
    interpreter.process_line("20 LET x = 5").unwrap();
    interpreter.process_line("30 END").unwrap();

    // Start execution
    interpreter.step(); // Execute line 10

    // Should NOT be able to edit during execution (state is Ready but execution_started = true)
    let edit_result = interpreter.process_line("40 PRINT 2");
    assert!(edit_result.is_err());
    assert!(
        edit_result
            .unwrap_err()
            .to_string()
            .contains("Cannot edit program while interpreter is running")
    );

    // Continue execution
    interpreter.step(); // Execute line 20
    interpreter.step(); // Execute line 30 (END) -> state becomes Finished

    // After execution finishes (Finished state), should be able to edit
    assert!(interpreter.process_line("40 PRINT 2").is_ok());
}

#[test]
fn test_edit_after_reset() {
    let mut interpreter = Interpreter::default();

    // Add lines and execute
    interpreter.process_line("10 PRINT 1").unwrap();
    interpreter.process_line("20 END").unwrap();

    interpreter.step(); // Execute line 10
    interpreter.step(); // Execute line 20 -> Finished

    // Reset
    interpreter.reset();

    // After reset, should be able to edit (execution_started = false)
    assert!(interpreter.process_line("30 PRINT 3").is_ok());
}

#[test]
fn test_edit_waiting_for_input() {
    let mut interpreter = Interpreter::default();

    // Add INPUT statement
    interpreter.process_line("10 INPUT x").unwrap();
    interpreter.process_line("20 END").unwrap();

    // Start execution
    interpreter.step(); // Execute line 10 -> WaitingForInput

    // Should NOT be able to edit while waiting for input
    let edit_result = interpreter.process_line("30 PRINT 3");
    assert!(edit_result.is_err());
    assert!(
        edit_result
            .unwrap_err()
            .to_string()
            .contains("Cannot edit program while interpreter is running")
    );

    // Provide input
    interpreter.provide_input("42").unwrap(); // State becomes Ready, execution_started = true

    // Still should NOT be able to edit (execution_started = true)
    let edit_result = interpreter.process_line("40 PRINT 4");
    assert!(edit_result.is_err());

    // Finish execution
    interpreter.step(); // Execute line 20 -> Finished

    // Now should be able to edit
    assert!(interpreter.process_line("50 PRINT 5").is_ok());
}

#[test]
fn test_edit_after_error() {
    let mut interpreter = Interpreter::default();

    // Add line with error (division by zero)
    interpreter.process_line("10 PRINT 5 / 0").unwrap();

    // Execute - should error
    interpreter.step(); // Division by zero -> Error state

    // Should be able to edit after error
    assert!(interpreter.process_line("20 PRINT 2").is_ok());
}

#[test]
fn test_execute_direct_print() {
    let mut interpreter = Interpreter::default();
    // Execute PRINT directly
    let batch = interpreter.execute("PRINT 2 + 2").unwrap();
    assert_eq!(batch.outputs, vec!["4"]);
    assert!(batch.errors.is_empty());
}

#[test]
fn test_execute_direct_let() {
    let mut interpreter = Interpreter::default();
    // LET statement
    let batch = interpreter.execute("LET x = 5").unwrap();
    assert!(batch.outputs.is_empty());
    // Check variable
    let vars = interpreter.get_variables();
    assert_eq!(vars.len(), 1);
    assert_eq!(vars[0].name, "x");
    assert_eq!(vars[0].value, "5");
}

#[test]
fn test_execute_direct_input() {
    let mut interpreter = Interpreter::default();
    // INPUT statement should wait for input
    let batch = interpreter.execute("INPUT y").unwrap();
    assert_eq!(batch.inputs, vec!["y"]);
    // State should be WaitingForInput
    assert_eq!(interpreter.get_state(), InterpreterState::WaitingForInput);
    // Provide input
    interpreter.provide_input("42").unwrap();
    // Variable should be set
    let vars = interpreter.get_variables();
    assert_eq!(vars.len(), 1);
    assert_eq!(vars[0].name, "y");
    assert_eq!(vars[0].value, "42");
}

#[test]
fn test_execute_direct_invalid_statement() {
    let mut interpreter = Interpreter::default();
    // GOTO not allowed in direct mode
    let result = interpreter.execute("GOTO 10");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not allowed"));
}
