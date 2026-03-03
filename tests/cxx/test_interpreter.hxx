#include "ffi/cxx/interpreter.hxx"
#include <QDebug>
#include <QSignalSpy>
#include <QTest>

class TestQBasicInterpreter : public QObject {
  Q_OBJECT

private slots:
  void initTestCase();
  void cleanupTestCase();

  void testProcessLine();
  void testLineDeletion();
  void testStepExecution();
  void testVariables();
  void testLineStats();
  void testSyntaxTree();
  void testClear();
  void testSignals();

private:
  QBasicInterpreter *interpreter = nullptr;
};
