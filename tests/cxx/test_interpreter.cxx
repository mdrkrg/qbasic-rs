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

// Include the moc generated file
#include "test_interpreter.moc"
