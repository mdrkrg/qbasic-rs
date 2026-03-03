#pragma once

#include "ffi/mod.rs.h"
#include <QObject>
#include <QString>
#include <vector>

class QBasicInterpreter : public QObject {
  Q_OBJECT

public:
  explicit QBasicInterpreter(QObject *parent = nullptr);

  ~QBasicInterpreter();

  /// Process user input, do the following things:
  /// 1. Deal with commands
  /// 2. If code starts with number, this is a program edit
  /// 3. Otherwise directly execute the code
  bool processInput(const QString &input) noexcept;

  /// Process a line edit command
  /// - If line is a number, delete the line (ignore if not exist)
  /// - Otherwise, add / update the line
  bool processLine(const QString &lineText) noexcept;

  /// Directly execute a line of QBasic code (without line number)
  bool executeDirect(const QString &lineText) noexcept;

  /// Clear all states and reset the interpreter
  void clear() noexcept;

  /// Load a file as source code and rebuild the interpreter
  bool loadFile(const QString &path) noexcept;

  /// Get program lines with syntax trees
  std::vector<qbasic_rs::ProgramLine> getProgramLines() const noexcept;

  /// Run the interpreter until interrupt (input event), error or finish
  void run() noexcept;

  /// Execute one interpreter step and return the side effects
  /// that this step will generate
  void step() noexcept;

  /// Reset interpreter state but keep program
  void reset() noexcept;

  /// Stop current execution (if running)
  void stop() noexcept;

  /// Provide input to the interpreter state if it is requiring it
  bool provideInput(const QString &value) noexcept;

  /// Get enum str of interpreter state
  QString getStateString() const noexcept;

  /// Get current line number of the interpreter
  quint32 getCurrentLine() const noexcept;

  /// Get the error message if the interpreter is in error state
  QString getErrorMessage() const noexcept;

  /// Get the name of the variable if the program is waiting for input
  QString getWaitingForInput() const noexcept;

  /// Get interpreter state
  qbasic_rs::InterpreterState getState() const noexcept;

  /// Check if program can be edited
  bool canEdit() const noexcept;

  /// Check if interpreter is currently running
  bool isRunning() const noexcept;

  /// Get current variables of the interpreter and usage count
  std::vector<qbasic_rs::Variable> getVariables() const noexcept;

  /// Get line statistics of lines
  std::vector<qbasic_rs::LineStats> getLineStats() const noexcept;

  /// Get syntax tree at line number
  QString getSyntaxTree(quint32 lineNo) const noexcept;

signals:
  /// An output has been received
  void outputReceived(const QString &text) noexcept;

  /// Request user input
  void inputRequested(const QString &varName) noexcept;

  /// An error has occurred
  void errorOccurred(const QString &error) noexcept;

  /// Execution has finished
  void executionFinished() noexcept;

  /// Program has changed
  void programChanged() noexcept;

  /// State has changed
  void stateChanged() noexcept;

  /// Statistics has been updated
  void statsUpdated() noexcept;

  /// Request loading a file
  void loadFileRequested() noexcept;

  /// Request quitting
  void quitRequested() noexcept;

private:
  /// Process a batch of events
  void processEventBatch(const qbasic_rs::EventBatch &batch) noexcept;

  /// Execute a single step (used by async run)
  void executeStep();

  rust::Box<qbasic_rs::Interpreter> m_interpreter;

  /// A flag for preventing nested run() calls
  bool m_isRunning = false;
};
