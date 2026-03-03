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
)

qbasic_rs = declare_dependency(
  link_args: [meson.current_source_dir() + '/target/debug/libqbasic_rs.a'],
)

qt = import('qt6')
qt_dep = dependency('qt6', modules: ['Core', 'Gui', 'Widgets'], required: true)

executable(
  'qbasic',
  [
    'src/ui/main.cc',
  ],
  dependencies: [
    qbasic_rs,
    qt_dep,
  ],
  include_directories: include_dirs,
  link_args: ['-Wl,--no-as-needed'],
)

# Tests
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
