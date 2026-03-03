project('qbasic-cxx', 'cpp', 'rust',
  version: '0.1.0',
  default_options: [
    'warning_level=3',
    'cpp_std=c++23',
  ],
)

include_dirs = include_directories(
  'src',
  'target/cxxbridge/qbasic-rs/src',
  'target/cxxbridge/rust',
)

qbasic_rs = declare_dependency(
  link_args: [meson.current_source_dir() + '/target/debug/libqbasic_rs.a'],
)

qt = import('qt6')
qt_dep = dependency('qt6', modules: ['Core', 'Gui', 'Widgets', 'Test'], required: true)

moc_files = qt.compile_moc(
  sources: [
    'src/ffi/cxx/interpreter.cxx',
  ],
  headers: [
    'src/ffi/cxx/interpreter.hxx',
  ],
)

executable(
  'qbasic',
  [
    'src/ui/main.cxx',
    'src/ffi/cxx/interpreter.cxx',
    moc_files,
  ],
  dependencies: [
    qbasic_rs,
    qt_dep,
  ],
  include_directories: include_dirs,
  link_args: ['-Wl,--no-as-needed'],
)

# Test executable
test_moc_files = qt.compile_moc(
  sources: [
    'tests/cxx/test_interpreter.cxx',
  ],
  headers: [
    'tests/cxx/test_interpreter.hxx',
  ],
)

test_executable = executable(
  'test_interpreter',
  [
    'tests/cxx/test_runner.cxx',
    'tests/cxx/test_interpreter.cxx',
    'src/ffi/cxx/interpreter.cxx',
    test_moc_files,
    moc_files,
  ],
  dependencies: [
    qbasic_rs,
    qt_dep,
  ],
  include_directories: include_dirs,
  link_args: ['-Wl,--no-as-needed'],
)

test('qt-tests', test_executable)

# Cargo tests
test('cargo-tests',
  find_program('cargo'),
  args: ['test'],
  workdir: meson.current_source_dir(),
)

# compile_commands.json
run_target('compile_commands',
  command: ['meson', 'compile_commands'],
  depends: [],
)
