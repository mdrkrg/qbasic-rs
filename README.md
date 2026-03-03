# QBasic

A Rust implementation of (a subset of) QBasic. The course project for SE1301.

## Quick Start

Requires `nix` to be installed as the package manager.

First start a development shell. This will install all the required dependencies:

```sh
nix develop
```

Or you can install Qt6 and cargo system wide. The required Qt6 modules are: Core, Gui, Widgets, Test.

The build commands requires `just` to be installed, or you can manually execute the build scripts provided in the `justfile`.

Building the project:

```sh
just setup
just compile
```

The executable will appear at `./build/qbasic`.

Running tests:

```sh
just test
# Or you can only run rust or Qt specific tests
just test qt-tests
just test cargo-tests
```

## Disclaimer

- Most of the unit tests are LLM generated.
- A large portion of the Qt frontend is LLM generated.
- A small portion of the Rust backend is LLM generated.
