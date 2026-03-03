#include "main_window.hxx"
#include <QDebug>
#include <QFileDialog>
#include <QHBoxLayout>
#include <QMessageBox>
#include <QStatusBar>
#include <QTimer>
#include <QVBoxLayout>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent), m_interpreter(new QBasicInterpreter(this)) {
  setWindowTitle("qbasic-rs");
  resize(800, 600);

  setupUi();
  connectSignals();

  // Initial updates
  updateProgramDisplay();
  updateSyntaxTreeDisplay();
  updateVariablesDisplay();
  updateStatsDisplay();
  onStateChanged();
}

void MainWindow::setupUi() {
  // Central widget and main layout
  QWidget *centralWidget = new QWidget(this);
  QVBoxLayout *mainLayout = new QVBoxLayout(centralWidget);

  // Top section: Code and Output side by side
  QHBoxLayout *topLayout = new QHBoxLayout();

  // Code display (left)
  {
    QVBoxLayout *codeLayout = new QVBoxLayout();
    codeLayout->addWidget(new QLabel("Code", this));
    m_codeDisplay = new QTextBrowser(this);
    m_codeDisplay->setReadOnly(true);
    codeLayout->addWidget(m_codeDisplay);
    topLayout->addLayout(codeLayout, 2); // 2 parts width
  }

  // Output display (right)
  {
    QVBoxLayout *outputLayout = new QVBoxLayout();
    outputLayout->addWidget(new QLabel("Execution Result", this));
    m_outputDisplay = new QTextBrowser(this);
    m_outputDisplay->setReadOnly(true);
    outputLayout->addWidget(m_outputDisplay);
    topLayout->addLayout(outputLayout, 1); // 1 part width
  }

  mainLayout->addLayout(topLayout, 3); // 3 parts height

  // Middle section: Syntax tree
  {
    QVBoxLayout *treeLayout = new QVBoxLayout();
    treeLayout->addWidget(new QLabel("Statements and Syntax Trees", this));
    m_treeDisplay = new QTextBrowser(this);
    m_treeDisplay->setReadOnly(true);
    m_treeDisplay->setFontFamily("Monospace");
    treeLayout->addWidget(m_treeDisplay);
    mainLayout->addLayout(treeLayout, 2); // 2 parts height
  }

  // Button row
  {
    QHBoxLayout *buttonLayout = new QHBoxLayout();
    m_btnLoad = new QPushButton("LOAD", this);
    m_btnRun = new QPushButton("RUN", this);
    m_btnClear = new QPushButton("CLEAR", this);

    buttonLayout->addWidget(m_btnLoad);
    buttonLayout->addWidget(m_btnRun);
    buttonLayout->addWidget(m_btnClear);
    buttonLayout->addStretch();

    mainLayout->addLayout(buttonLayout);
  }

  // Command line input
  {
    QVBoxLayout *cmdLayout = new QVBoxLayout();
    cmdLayout->addWidget(new QLabel("Command Input", this));
    m_cmdLineEdit = new QLineEdit(this);
    m_cmdLineEdit->setMinimumHeight(35);
    m_cmdLineEdit->setFont(QFont("Monospace", 12));
    m_cmdLineEdit->setPlaceholderText("Enter QBasic command or statement...");
    cmdLayout->addWidget(m_cmdLineEdit);
    QTimer::singleShot(0, m_cmdLineEdit,
                       [this]() { m_cmdLineEdit->setFocus(); });

    mainLayout->addLayout(cmdLayout);
  }

  // Status label
  m_statusLabel = new QLabel(this);
  statusBar()->addWidget(m_statusLabel);

  setCentralWidget(centralWidget);
}

void MainWindow::connectSignals() {
  // Command line
  connect(m_cmdLineEdit, &QLineEdit::returnPressed, this,
          &MainWindow::onCommandLineEntered);

  // Buttons
  connect(m_btnRun, &QPushButton::clicked, this, &MainWindow::onRunClicked);
  connect(m_btnClear, &QPushButton::clicked, this, &MainWindow::onClearClicked);
  connect(m_btnLoad, &QPushButton::clicked, this, &MainWindow::onLoadClicked);

  // Interpreter signals
  connect(m_interpreter, &QBasicInterpreter::outputReceived, this,
          &MainWindow::onOutputReceived);
  connect(m_interpreter, &QBasicInterpreter::errorOccurred, this,
          &MainWindow::onErrorOccurred);
  connect(m_interpreter, &QBasicInterpreter::inputRequested, this,
          &MainWindow::onInputRequested);
  connect(m_interpreter, &QBasicInterpreter::programChanged, this,
          &MainWindow::onProgramChanged);
  connect(m_interpreter, &QBasicInterpreter::stateChanged, this,
          &MainWindow::onStateChanged);
  connect(m_interpreter, &QBasicInterpreter::executionFinished, this,
          &MainWindow::onExecutionFinished);
  connect(m_interpreter, &QBasicInterpreter::loadFileRequested, this,
          &MainWindow::onLoadFileRequested);
  connect(m_interpreter, &QBasicInterpreter::quitRequested, this,
          &MainWindow::onQuitRequested);
  connect(m_interpreter, &QBasicInterpreter::statsUpdated, this,
          &MainWindow::updateStatsDisplay);
}

void MainWindow::onCommandLineEntered() {
  QString text = m_cmdLineEdit->text().trimmed();
  if (text.isEmpty()) {
    return;
  }

  m_cmdLineEdit->clear();

  if (m_waitingForInput) {
    // Provide input to interpreter
    if (m_interpreter->provideInput(text)) {
      m_waitingForInput = false;
      m_cmdLineEdit->setPlaceholderText("Enter QBasic command or statement...");
      m_statusLabel->setText("Input provided for " + m_inputVarName);
      // WARN: we need a clean distinction between run and step
      m_interpreter->run();
    } else {
      m_statusLabel->setText("Failed to provide input for " + m_inputVarName);
    }
    return;
  }

  // Normal command processing
  if (m_interpreter->processInput(text)) {
    m_statusLabel->setText("Command processed: " + text.left(30));
  } else {
    m_statusLabel->setText("Failed to process: " + text.left(30));
  }
}

void MainWindow::onRunClicked() { m_interpreter->run(); }

void MainWindow::onClearClicked() {
  m_interpreter->clear();
  // clear output
  m_outputDisplay->clear();
}

void MainWindow::onLoadClicked() {
  QString fileName = QFileDialog::getOpenFileName(
      this, "Load QBasic Program", "", "QBasic Files (*.bas);;All Files (*.*)");

  if (!fileName.isEmpty()) {
    if (m_interpreter->loadFile(fileName)) {
      m_statusLabel->setText("Loaded: " + fileName);
      // clear output
      m_outputDisplay->clear();
    } else {
      m_statusLabel->setText("Failed to load: " + fileName);
    }
  }
}

void MainWindow::onOutputReceived(const QString &text) {
  m_outputDisplay->append(text);
}

void MainWindow::onErrorOccurred(const QString &error) {
  m_outputDisplay->append("<font color='red'>Error: " + error + "</font>");
  m_statusLabel->setText("Error: " + error.left(50));
}

void MainWindow::onInputRequested(const QString &varName) {
  m_waitingForInput = true;
  m_inputVarName = varName;
  m_cmdLineEdit->setPlaceholderText("? ");
  m_cmdLineEdit->clear();
  m_statusLabel->setText("Waiting for input: " + varName);

  // Set command line focus
  m_cmdLineEdit->setFocus();
}

void MainWindow::onProgramChanged() {
  updateProgramDisplay();
  updateSyntaxTreeDisplay();
}

void MainWindow::onStateChanged() {
  QString state = m_interpreter->getStateString();
  m_statusLabel->setText("State: " + state);

  // Update button states
  bool canEdit = m_interpreter->canEdit();
  m_btnRun->setEnabled(canEdit);
  m_btnClear->setEnabled(canEdit);
  m_btnLoad->setEnabled(canEdit);
  m_cmdLineEdit->setEnabled(canEdit || m_waitingForInput);
}

void MainWindow::onExecutionFinished() {
  m_statusLabel->setText("Execution finished");
}

void MainWindow::onLoadFileRequested() {
  // LOAD command without filename
  // Show file dialog
  onLoadClicked();
}

void MainWindow::onQuitRequested() {
  QMessageBox::information(this, "Quit",
                           "QUIT command received. Closing application.");
  close();
}

void MainWindow::updateProgramDisplay() {
  auto lines = m_interpreter->getProgramLines();
  m_codeDisplay->clear();

  for (const auto &line : lines) {
    QString lineText = QString::fromStdString(std::string(line.text));
    m_codeDisplay->append(lineText);
  }
}

void MainWindow::updateSyntaxTreeDisplay() {
  auto lines = m_interpreter->getProgramLines();
  m_treeDisplay->clear();

  for (const auto &line : lines) {
    QString syntaxTree = m_interpreter->getSyntaxTree(line.lineno);
    if (!syntaxTree.isEmpty()) {
      m_treeDisplay->append("Line " + QString::number(line.lineno) + ":");
      m_treeDisplay->append(syntaxTree);
      m_treeDisplay->append("");
    }
  }
}

void MainWindow::updateVariablesDisplay() {
  // TODO: Variables display panel
}

void MainWindow::updateStatsDisplay() {
  // TODO: Statistics display panel
}
