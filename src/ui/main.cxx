#include "main_window.hxx"
#include <QApplication>
#include <QStyleFactory>

int main(int argc, char *argv[]) {
  QApplication app(argc, argv);

  app.setApplicationName("qbasic-rs");
  app.setOrganizationName("qbasic-rs");

  app.setStyle(QStyleFactory::create("Fusion"));

  MainWindow window;
  window.show();

  return app.exec();
}
