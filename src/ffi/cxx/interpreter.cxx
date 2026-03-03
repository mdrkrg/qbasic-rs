#include "interpreter.hxx"
#include "cxx.h"
#include "ffi/cxx/utils.hxx"
#include "ffi/mod.rs.h"
#include "qlogging.h"
#include <QDebug>
#include <QFile>
#include <QFileInfo>
#include <format>
#include <vector>

QBasicInterpreter::QBasicInterpreter(QObject *parent)
    : QObject(parent), m_interpreter(qbasic_rs::new_interpreter()) {}

QBasicInterpreter::~QBasicInterpreter() = default;

bool QBasicInterpreter::processLine(const QString &lineText) noexcept {
  try {
    const auto result = m_interpreter->process_line(lineText.toStdString());
    emit programChanged();

    // Log operation type
    switch (result.op_type) {
    case qbasic_rs::LineOpType::Added:
      qDebug() << std::format("Line {} added / updated", result.lineno);
      break;
    case qbasic_rs::LineOpType::Deleted:
      qDebug() << std::format("Line {} deleted", result.lineno);
      break;
    }

    return true;
  } catch (const rust::Error &e) {
    qWarning() << std::format("Failed to process line: {}", e.what());
    emit errorOccurred(QString::fromUtf8(e.what()));
    return false;
  }
}

bool QBasicInterpreter::executeDirect(const QString &lineText) noexcept {
  try {
    const auto batch = m_interpreter->execute(lineText.toStdString());
    processEventBatch(batch);
    return true;
  } catch (const rust::Error &e) {
    qWarning() << std::format("Failed to execute direct: {}", e.what());
    emit errorOccurred(QString::fromUtf8(e.what()));
    return false;
  }
}

void QBasicInterpreter::clear() noexcept {
  m_interpreter->clear();
  emit programChanged();
  emit stateChanged();
  emit statisticsUpdated();
}

bool QBasicInterpreter::loadFile(const QString &path) noexcept {
  if (!QFile::exists(path)) {
    emit errorOccurred("File does not exist: " + path);
    return false;
  }

  try {
    m_interpreter->load_file(path.toStdString());
    emit programChanged();
    return true;
  } catch (const rust::Error &e) {
    qWarning() << std::format("Failed to load file: {}", e.what());
    emit errorOccurred(QString::fromUtf8(e.what()));
    return false;
  }
}

std::vector<qbasic_rs::ProgramLine>
QBasicInterpreter::getProgramLines() const noexcept {
  return rust::vec_to_cxx(m_interpreter->get_program_lines());
}

void QBasicInterpreter::run() noexcept {
  if (m_isRunning) {
    qWarning() << "Interpreter is already running";
    return;
  }

  m_isRunning = true;
  emit stateChanged();

  const auto batch = m_interpreter->run();
  processEventBatch(batch);

  m_isRunning = false;
  emit stateChanged();
  emit statisticsUpdated();
}

void QBasicInterpreter::step() noexcept {
  if (m_isRunning) {
    qWarning() << "Cannot step while running";
    return;
  }

  const auto batch = m_interpreter->step();
  processEventBatch(batch);
  emit stateChanged();
  emit statisticsUpdated();
}

void QBasicInterpreter::reset() noexcept {
  m_interpreter->reset();
  m_isRunning = false;
  emit stateChanged();
  emit statisticsUpdated();
}

bool QBasicInterpreter::provideInput(const QString &value) noexcept {
  try {
    m_interpreter->provide_input(value.toStdString());
    emit stateChanged();
    return true;
  } catch (const rust::Error &e) {
    qWarning() << std::format("Failed to provide input: {}", e.what());
    emit errorOccurred(QString::fromUtf8(e.what()));
    return false;
  }
}

QString QBasicInterpreter::getStateString() const noexcept {
  switch (getState()) {
  case qbasic_rs::InterpreterState::Ready:
    return "Ready";
  case qbasic_rs::InterpreterState::WaitingForInput:
    return "WaitingForInput";
  case qbasic_rs::InterpreterState::Finished:
    return "Finished";
  case qbasic_rs::InterpreterState::Error:
    return "Error";
  }
}

quint32 QBasicInterpreter::getCurrentLine() const noexcept {
  return m_interpreter->get_current_line();
}

QString QBasicInterpreter::getErrorMessage() const noexcept {
  return QString::fromStdString(
      std::string{m_interpreter->get_error_message()});
}

QString QBasicInterpreter::getWaitingForInput() const noexcept {
  return QString::fromStdString(
      std::string{m_interpreter->get_waiting_for_input()});
}

std::vector<qbasic_rs::Variable>
QBasicInterpreter::getVariables() const noexcept {
  return rust::vec_to_cxx(m_interpreter->get_variables());
}

std::vector<qbasic_rs::LineStats>
QBasicInterpreter::getLineStats() const noexcept {
  return rust::vec_to_cxx(m_interpreter->get_line_stats());
}

QString QBasicInterpreter::getSyntaxTree(quint32 lineNo) const noexcept {
  try {
    return QString::fromStdString(
        std::string{m_interpreter->get_syntax_tree(lineNo)});
  } catch (const rust::Error &e) {
    qWarning() << std::format("Failed to get syntax tree: {}", e.what());
    return "";
  }
}

qbasic_rs::InterpreterState QBasicInterpreter::getState() const noexcept {
  return m_interpreter->get_state();
}

bool QBasicInterpreter::canEdit() const noexcept {
  return m_interpreter->can_edit();
}

void QBasicInterpreter::processEventBatch(
    const qbasic_rs::EventBatch &batch) noexcept {
  for (const auto &output : batch.outputs) {
    emit outputReceived(QString::fromStdString(std::string{output}));
  }

  for (const auto &input : batch.inputs) {
    emit inputRequested(QString::fromStdString(std::string{input}));
  }

  for (const auto &error : batch.errors) {
    emit errorOccurred(QString::fromStdString(std::string{error}));
  }

  for (const auto &debug : batch.debug_messages) {
    qDebug() << "Debug:" << QString::fromStdString(std::string{debug});
  }

  if (batch.finished) {
    emit executionFinished();
  }
}
