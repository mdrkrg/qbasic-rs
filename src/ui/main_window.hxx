#pragma once

#include "ffi/cxx/interpreter.hxx"
#include <QLabel>
#include <QLineEdit>
#include <QMainWindow>
#include <QPushButton>
#include <QTextBrowser>

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  explicit MainWindow(QWidget *parent = nullptr);
  ~MainWindow() override = default;

private slots:
  void onCommandLineEntered();
  void onRunClicked();
  void onClearClicked();
  void onLoadClicked();

  void onOutputReceived(const QString &text);
  void onErrorOccurred(const QString &error);
  void onInputRequested(const QString &varName);
  void onProgramChanged();
  void onStateChanged();
  void onExecutionFinished();
  void onLoadFileRequested();
  void onQuitRequested();

private:
  void setupUi();
  void connectSignals();
  void updateProgramDisplay();
  void updateSyntaxTreeDisplay();
  void updateVariablesDisplay();
  void updateStatsDisplay();

  // UI widgets
  QTextBrowser *m_codeDisplay = nullptr;
  QTextBrowser *m_outputDisplay = nullptr;
  QTextBrowser *m_treeDisplay = nullptr;
  QPushButton *m_btnLoad = nullptr;
  QPushButton *m_btnRun = nullptr;
  QPushButton *m_btnClear = nullptr;
  QLineEdit *m_cmdLineEdit = nullptr;
  QLabel *m_statusLabel = nullptr;

  // Interpreter
  QBasicInterpreter *m_interpreter = nullptr;

  // State for INPUT handling
  bool m_waitingForInput = false;
  QString m_inputVarName;
};
