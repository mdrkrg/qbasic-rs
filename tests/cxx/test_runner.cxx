#include <QCoreApplication>
#include <QTest>

#include "test_interpreter.hxx"

int main(int argc, char *argv[]) {
  QCoreApplication app(argc, argv);

  app.setApplicationName("QBasic Tests");

  int result = 0;

  TestQBasicInterpreter test1;
  result |= QTest::qExec(&test1, argc, argv);

  return result;
}
