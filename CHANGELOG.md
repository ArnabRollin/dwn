# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.11.0] - 2023-10-27

### Added

- Arrays
- Array pretty-print function (`format_array`)

### Changed

- Literals now return the value provided (if there is no function or operator involved) instead of `None`.

### Removed

- Unnecessary `match` in `say` and `short_say` functions.

## [0.10.0] - 2023-10-26

### Added

- Variable-options in CLI
- Bytecode compiler and runner (level 1)
- Better documentation in CLI.

### Changed

- Now there are two runner functions: `run_tokens` and `run` (which tokenizes the line and calls `run_tokens` with the tokens)
- Integers are now `i64` and floats are `f64`

## [0.9.0] - 2023-10-24

### Added

- `!=`, `lazy=` and `lazy!=` operators
- `vars` function which prints all the variables
- `int` and `float` converter functions
- Guessing game example to `README.md`

### Changed

- Changed the example in `README.md`.

### Fixed

- Scope build-up bug has now been fixed

## [0.8.0] - 2023-10-21

### Added

- Nested scopes
- `break` statement
- Scopes and if statements now return the value of the last statement.

## [0.7.0] - 2023-10-18

### Added

- `while` and `until` loops
- `==`, `>` and `<` compare operators
- `+=`, `-=`, `*=` and `/=` "operate and assign" operators
- Functions now say their name on error.

## [0.6.1] - 2023-10-14

### Fixed

- Bug where variables can't be used with operators

## [0.6.0] - 2023-10-14

### Added

- Boolean type (`true` and `false`)
- `if` function

### Fixed

- Fixed bug where two arguments cannot be provided to scope-accepting function.

## [0.5.0] - 2023-10-13

### Changed

- The change log has been remodeled.

## [0.4.0] - 2023-10-13

### Added

- Documentation
- Variables now drop at scope end
- Forever loop
- Scope runner
- None type

### Removed

- Examples removed from `src` folder and embedded in `README.md`.

## [0.3.0] - 2023-10-8

### Added

- Float and integer support

### Changed

- Error reporting uses Result instead to (Option, Option).

## [0.2.0] - 2023-10-5

### Added

- Rust workflow to test and build Dawn (dwn).
- README and CHANGELOG files.
- IDLE (Integrated Learning and Development Environment)s
- Variable creations
- Framework creation for extensions.
- Examples folder.
- Tests.

### Changed

- Remade lexer from scratch.
- Changed interpreter API.

## [0.1.0] - 2023-09-12

### Added

- Simple function and string parsing

[unreleased]: https://github.com/ArnabRollin/dwn/compare/v0.11.0...HEAD

[0.11.0]: https://github.com/ArnabRollin/dwn/compare/v0.10.0...v0.11.0

[0.10.0]: https://github.com/ArnabRollin/dwn/compare/v0.9.0...v0.10.0

[0.9.0]: https://github.com/ArnabRollin/dwn/compare/v0.8.0...v0.9.0

[0.8.0]: https://github.com/ArnabRollin/dwn/compare/v0.7.0...v0.8.0

[0.7.0]: https://github.com/ArnabRollin/dwn/compare/v0.6.1...v0.7.0

[0.6.1]: https://github.com/ArnabRollin/dwn/compare/v0.6.0...v0.6.1

[0.6.0]: https://github.com/ArnabRollin/dwn/compare/v0.5.0...v0.6.0

[0.5.0]: https://github.com/ArnabRollin/dwn/compare/v0.4.0...v0.5.0

[0.4.0]: https://github.com/ArnabRollin/dwn/compare/v0.3.0...v0.4.0

[0.3.0]: https://github.com/ArnabRollin/dwn/compare/v0.2.0...v0.3.0

[0.2.0]: https://github.com/ArnabRollin/dwn/compare/v0.1.0..v0.2.0

[0.1.0]: https://github.com/ArnabRollin/dwn/releases/tag/v0.1.0
