#include "cxx.h"

namespace rust {
/// Convert rust Vec to std::vector
template <typename T> auto vec_to_cxx(rust::Vec<T> rust_vec) -> std::vector<T> {
  std::vector<T> cxx_vec{};
  std::copy(rust_vec.begin(), rust_vec.end(), std::back_inserter(cxx_vec));
  return cxx_vec;
}
} // namespace rust
