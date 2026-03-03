#pragma once
#include "cxx.h"

namespace rust {
/// Convert rust Vec to std::vector
template <typename T> auto vec_to_cxx(rust::Vec<T> rust_vec) -> std::vector<T> {
  std::vector<T> cxx_vec{};
  std::copy(rust_vec.begin(), rust_vec.end(), std::back_inserter(cxx_vec));
  return cxx_vec;
}
} // namespace rust

namespace utils {
/// Split a string into vector
/// Reference: <https://researchdatapod.com/string-split-cpp/>
inline std::vector<std::string> split_view_into(std::string_view str,
                                                char delimiter) {
  std::vector<std::string> output{};
  size_t start = 0; // Starting index of the current token

  // Reserve approximate space to minimize reallocations
  output.reserve(std::count(str.begin(), str.end(), delimiter) + 1);

  // Iterate through the string to find delimiters
  while (start < str.size()) {
    const auto end = str.find(delimiter, start); // Find the next delimiter
    if (start != end) {
      // Add the substring (view) from start to the delimiter
      output.emplace_back(str.substr(start, end - start));
    }
    if (end == std::string_view::npos)
      break;         // Exit loop if no more delimiters are found
    start = end + 1; // Move the start to the character after the delimiter
  }

  return output;
}
} // namespace utils
