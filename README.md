# brain

brain is a high level programming language that compiles into brainfuck. It is
implemented using the Rust programming language.

[brainfuck][brainfuck] is an esoteric programming language with 8 very simple
instructions. Unfortunately, brainfuck code is quite difficult to understand and
write. brain makes it easier to write brainfuck programs by allowing you to
write in a more familiar syntax. The compiler is designed to optimize the
generated brainfuck code as much as possible.

## Optimization Goals

1. Generate small brainfuck files (use as few instructions as possible)
2. Generate memory efficient code (use as few brainfuck cells as possible)

As the project reaches 1.0.0, these goals will become more expressed in the
compiled output of the program.

## Usage

This project contains both the brain compiler and a basic brainfuck interpreter.

Make sure you have [rust][rust] and cargo (comes with rust) installed.

### brain compiler

To compile a brain (.brn) file into brainfuck (.bf)
```
cargo run -q --bin brain -- filename.brn
```
where `filename.brn` is the brain program you want to compile

Use `--help` to see further options and additional information
```
cargo run -q --bin brain -- --help
```

**If the brain compiler seems to be taking too long or "hanging", try running
`cargo build` first to see if the rust compiler is just taking too long for
some reason.**

### brainfuck interpreter

The brain compiler only officially targets this brainfuck interpreter. You may
experience varying results with other brainfuck interpreters/compilers. There
really isn't a definitive spec on how brainfuck should behave so it is just
easier to have a static compilation target that won't vary in how it behaves.

To run brainfuck programs:
```
cargo run -q --bin brainfuck -- filename
```
where `filename` is the brainfuck program you want to run

## Examples

There are various brain examples in the `examples/` directory which you can
compile into brainfuck using the usage instructions above.

[brainfuck]: http://www.muppetlabs.com/~breadbox/bf/
[rust]: https://www.rust-lang.org/
