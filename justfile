setup:
  cargo build
  meson setup build --reconfigure

compile: setup
  meson compile -C build

test *args: compile
  meson test -C build {{args}}
