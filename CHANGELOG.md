# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
### Added
- This CHANGELOG.md file
- Branching with `if`, `else if` and `else` statements
- Comparison operators like `==`, `!=`, `>=`, etc.
- Declaration without initialization
  - Allows you to defer initialization to whenever makes sense in your code
  - Note that static checking is not fully functional
    (see [#7](https://github.com/brain-lang/brain/issues/7))
- `mut` keyword allows you to declare mutable variables (see examples)
  - This is currently not checked or enforced by the compiler, however it will
    be when [#64](https://github.com/brain-lang/brain/issues/64) is implemented
  - It is **highly** recommended that you adopt `mut` early to avoid breakage
- Simple wrapping unsigned 8-bit `u8` type
- Much cleaner and easier to build on codebase underneath the compiler which
  means more features sooner!

### Changed
- Brand new syntax based on Rust but adapted to suit the needs of brain

### Removed
- The `in` and `out` statements are gone and replaced with new method calls on
  global `stdin` and `stdout` objects (see examples for details)

## [0.1.2] - 2017-02-03
### Fixed
- **SEVERE BUG:** Incomplete code generation in while loops was not ensuring that
  pointer ended up at the result of the while loop expression

## [0.1.1] - 2017-01-29
### Fixed
- Bug in optimizations which was removing loops that should not be removed

## [0.1.0] - 2017-01-28
### Added
- First release with no partially implemented code generation
- While loops
- Reading input
- Writing output
- Variable declarations

## 0.0.1 - 2017-01-22
### Added
- Basic barely-functional compilation of some output statements and basic
  declarations
- Some features still partially unimplemented and will panic if used

[Unreleased]: https://github.com/brain-lang/brain/compare/v0.1.2...develop
[0.1.2]: https://github.com/brain-lang/brain/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/brain-lang/brain/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/brain-lang/brain/compare/v0.0.1...v0.1.0
