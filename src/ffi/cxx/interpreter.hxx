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

  /// Process a line edit command
  /// - If line is a number, delete the line (ignore if not exist)
  /// - Otherwise, add / update the line
  bool processLine(const QString &lineText) noexcept;
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

  /// Get current variables of the interpreter and usage count
  std::vector<qbasic_rs::Variable> getVariables() const noexcept;
  /// Get line statistics of lines
  std::vector<qbasic_rs::LineStats> getLineStats() const noexcept;
  /// Get syntax tree at line number
  QString getSyntaxTree(quint32 lineNo) const noexcept;

signals:
  void outputReceived(const QString &text) noexcept;
  void inputRequested(const QString &varName) noexcept;
  void errorOccurred(const QString &error) noexcept;
  void executionFinished() noexcept;
  void programChanged() noexcept;
  void stateChanged() noexcept;
  void statisticsUpdated() noexcept;

private:
  /// Process a batch of events
  void processEventBatch(const qbasic_rs::EventBatch &batch) noexcept;

  rust::Box<qbasic_rs::Interpreter> m_interpreter;
  /// A flag for preventing nested run() calls
  bool m_isRunning = false;
};
