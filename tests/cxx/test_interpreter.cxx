#include "test_interpreter.hxx"
#include "ffi/cxx/interpreter.hxx"
#include <QDebug>
#include <QSignalSpy>
#include <QTest>
#include <vector>

void TestQBasicInterpreter::initTestCase() {
  interpreter = new QBasicInterpreter(this);
  QVERIFY(interpreter != nullptr);
}

void TestQBasicInterpreter::cleanupTestCase() {
  delete interpreter;
  interpreter = nullptr;
}

void TestQBasicInterpreter::testProcessLine() {
  qDebug() << "Testing processLine...";

  // Test adding lines
  QVERIFY(interpreter->processLine("10 PRINT 2 + 2"));
  QVERIFY(interpreter->processLine("15 LET x = 10"));
  QVERIFY(interpreter->processLine("20 LET x = 5"));
  QVERIFY(interpreter->processLine("30 PRINT x * 2"));

  auto lines = interpreter->getProgramLines();
  QCOMPARE(lines.size(), 4);

  // Verify line numbers and content
  QCOMPARE(lines[0].lineno, 10u);
  QCOMPARE(std::string(lines[0].text), "10 PRINT 2 + 2");
  QCOMPARE(lines[1].lineno, 15u);
  QCOMPARE(std::string(lines[1].text), "15 LET x = 10");
  QCOMPARE(lines[2].lineno, 20u);
  QCOMPARE(std::string(lines[2].text), "20 LET x = 5");
  QCOMPARE(lines[3].lineno, 30u);
  QCOMPARE(std::string(lines[3].text), "30 PRINT x * 2");
}

void TestQBasicInterpreter::testLineDeletion() {
  qDebug() << "Testing line deletion...";

  // Delete line 20
  QVERIFY(interpreter->processLine("20"));

  auto lines = interpreter->getProgramLines();
  QCOMPARE(lines.size(), 3);

  // Verify only lines 10 and 30 remain
  QCOMPARE(lines[0].lineno, 10u);
  QCOMPARE(lines[1].lineno, 15u);
  QCOMPARE(lines[2].lineno, 30u);
}

void TestQBasicInterpreter::testStepExecution() {
  qDebug() << "Testing step execution...";

  // Reset interpreter state
  interpreter->reset();

  // Step through execution
  interpreter->step(); // Execute line 10
  interpreter->step(); // Execute line 15
  interpreter->step(); // Execute line 30 (line 20 was deleted)

  // Verify execution happened (basic check)
  // Note: More detailed checks would require output signal verification
  QVERIFY(true); // Placeholder for now
}

void TestQBasicInterpreter::testVariables() {
  qDebug() << "Testing variables...";

  auto variables = interpreter->getVariables();

  // After executing LET x = 10, we should have variable x
  bool foundX = false;
  for (const auto &var : variables) {
    if (std::string(var.name) == "x") {
      foundX = true;
      // Value should be "10" (as string)
      QCOMPARE(std::string(var.value), "10");
      break;
    }
  }
  QVERIFY(foundX);
}

void TestQBasicInterpreter::testLineStats() {
  qDebug() << "Testing line statistics...";

  auto stats = interpreter->getLineStats();

  // Lines 10, 15 and 30 should have execution counts
  bool foundLine10 = false;
  bool foundLine15 = false;
  bool foundLine30 = false;

  for (const auto &stat : stats) {
    if (stat.lineno == 10) {
      foundLine10 = true;
      QVERIFY(stat.execution_count > 0);
    } else if (stat.lineno == 15) {
      foundLine15 = true;
      QVERIFY(stat.execution_count > 0);
    } else if (stat.lineno == 30) {
      foundLine30 = true;
      QVERIFY(stat.execution_count > 0);
    }
  }

  QVERIFY(foundLine10);
  QVERIFY(foundLine15);
  QVERIFY(foundLine30);
}

void TestQBasicInterpreter::testSyntaxTree() {
  qDebug() << "Testing syntax tree...";

  auto lines = interpreter->getProgramLines();

  for (const auto &line : lines) {
    QString syntaxTree = interpreter->getSyntaxTree(line.lineno);
    QVERIFY(!syntaxTree.isEmpty());
    qDebug() << "Line" << line.lineno << "syntax tree:" << syntaxTree;
  }
}

void TestQBasicInterpreter::testClear() {
  qDebug() << "Testing clear...";

  interpreter->clear();

  auto lines = interpreter->getProgramLines();
  QCOMPARE(lines.size(), 0);

  auto variables = interpreter->getVariables();
  QCOMPARE(variables.size(), 0);
}

void TestQBasicInterpreter::testSignals() {
  qDebug() << "Testing signals...";

  // Test output signal
  QSignalSpy outputSpy(interpreter, &QBasicInterpreter::outputReceived);
  QSignalSpy errorSpy(interpreter, &QBasicInterpreter::errorOccurred);
  QSignalSpy programSpy(interpreter, &QBasicInterpreter::programChanged);
  QSignalSpy stateSpy(interpreter, &QBasicInterpreter::stateChanged);

  // Add a line to trigger programChanged
  interpreter->processLine("40 PRINT \"Test\"");
  QVERIFY(programSpy.count() > 0);

  // Reset to trigger stateChanged
  interpreter->reset();
  QVERIFY(stateSpy.count() > 0);

  // Clear program
  interpreter->clear();
  QVERIFY(programSpy.count() > 1); // Should have been triggered again
}

void TestQBasicInterpreter::testCommandParsing() {
  qDebug() << "Testing command parsing...";

  // Clear any existing state
  interpreter->clear();

  // Test CLEAR command
  {
    // First add some program lines
    QVERIFY(interpreter->processLine("10 PRINT 1"));
    QVERIFY(interpreter->processLine("20 PRINT 2"));
    auto lines = interpreter->getProgramLines();
    QCOMPARE(lines.size(), 2);

    // Execute CLEAR command
    QVERIFY(interpreter->processInput("CLEAR"));
    lines = interpreter->getProgramLines();
    QCOMPARE(lines.size(), 0);
  }

  // Test RUN command
  {
    // Clear and add a simple program
    interpreter->clear();
    QVERIFY(interpreter->processLine("10 PRINT \"Hello\""));
    QVERIFY(interpreter->processLine("20 END"));

    // Spy on output signal
    QSignalSpy outputSpy(interpreter, &QBasicInterpreter::outputReceived);

    // Execute RUN command
    QVERIFY(interpreter->processInput("RUN"));

    // Should have output
    QTRY_VERIFY(outputSpy.count() > 0);
    QCOMPARE(outputSpy.first()[0].toString(), "Hello");
  }

  // Test HELP command
  {
    QSignalSpy helpSpy(interpreter, &QBasicInterpreter::outputReceived);
    QVERIFY(interpreter->processInput("HELP"));
    QTRY_VERIFY(helpSpy.count() > 0);
    QString helpText = helpSpy.first()[0].toString();
    QVERIFY(helpText.contains("RUN"));
    QVERIFY(helpText.contains("LOAD"));
    QVERIFY(helpText.contains("CLEAR"));
    QVERIFY(helpText.contains("HELP"));
    QVERIFY(helpText.contains("QUIT"));
  }

  // Test QUIT command (should emit signal)
  {
    QSignalSpy quitSpy(interpreter, &QBasicInterpreter::quitRequested);
    QVERIFY(interpreter->processInput("QUIT"));
    QTRY_VERIFY(quitSpy.count() > 0);
  }

  // Test LOAD command without filename (should emit signal)
  {
    QSignalSpy loadSpy(interpreter, &QBasicInterpreter::loadFileRequested);
    QVERIFY(interpreter->processInput("LOAD"));
    QTRY_VERIFY(loadSpy.count() > 0);
  }

  // Test LOAD command with filename (will fail because file doesn't exist)
  {
    // This should fail because file doesn't exist
    QVERIFY(!interpreter->processInput("LOAD nonexistent.bas"));
  }

  // Test direct statement execution (PRINT without line number)
  {
    QSignalSpy outputSpy(interpreter, &QBasicInterpreter::outputReceived);
    QVERIFY(interpreter->processInput("PRINT 3 + 4"));
    QTRY_VERIFY(outputSpy.count() > 0);
    QCOMPARE(outputSpy.first()[0].toString(), "7");
  }

  // Test direct statement execution (LET without line number)
  {
    QVERIFY(interpreter->processInput("LET x = 42"));
    auto variables = interpreter->getVariables();
    QCOMPARE(variables.size(), 1);
    QCOMPARE(std::string(variables[0].name), "x");
    QCOMPARE(std::string(variables[0].value), "42");
  }

  // Test direct statement execution (INPUT without line number)
  {
    QSignalSpy inputSpy(interpreter, &QBasicInterpreter::inputRequested);
    QVERIFY(interpreter->processInput("INPUT y"));
    QTRY_VERIFY(inputSpy.count() > 0);
    QCOMPARE(inputSpy.first()[0].toString(), "y");

    // Provide input
    QVERIFY(interpreter->provideInput("99"));
    auto variables = interpreter->getVariables();
    bool foundY = false;
    for (const auto &var : variables) {
      if (std::string(var.name) == "y") {
        foundY = true;
        QCOMPARE(std::string(var.value), "99");
        break;
      }
    }
    QVERIFY(foundY);
  }

  // Test case-insensitive commands
  {
    QVERIFY(interpreter->processInput("run")); // lowercase
    QVERIFY(interpreter->processInput("Run")); // mixed case
    QVERIFY(interpreter->processInput("RUN")); // uppercase
  }

  // Test line number editing (should be detected and routed to processLine)
  {
    QVERIFY(interpreter->processInput("50 PRINT \"Line edit\""));
    auto lines = interpreter->getProgramLines();
    bool foundLine50 = false;
    for (const auto &line : lines) {
      if (line.lineno == 50) {
        foundLine50 = true;
        QCOMPARE(std::string(line.text), "50 PRINT \"Line edit\"");
        break;
      }
    }
    QVERIFY(foundLine50);
  }

  // Test invalid command (should fall through to direct execution or fail)
  {
    // "GOTO 10" is not allowed in direct mode, should fail
    QVERIFY(!interpreter->processInput("GOTO 10"));
  }

  qDebug() << "Command parsing tests completed";
}

// Include the moc generated file
#include "test_interpreter.moc"
