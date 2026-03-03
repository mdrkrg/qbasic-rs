#include "ffi.rs.h"
#include <print>

int main() {
  const auto &interpreter = qbasic_rs::new_interpreter();
  std::println("Successfully created an interpreter!");
}
