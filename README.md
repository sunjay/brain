# brain

[![Crates.io](https://img.shields.io/crates/v/brain.svg)](https://crates.io/crates/brain)
[![Crates.io](https://img.shields.io/crates/l/brain.svg)](https://crates.io/crates/brain)
[![Build Status](https://travis-ci.org/brain-lang/brain.svg?branch=master)](https://travis-ci.org/brain-lang/brain)
[![Gitter](https://img.shields.io/gitter/room/brain-lang/brain.svg)](https://gitter.im/brain-lang/brain)

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

## brain syntax

For full examples, please see the `examples/` directory. Some
examples aren't fully implemented yet in the compiler.

The following examples are all working syntax:

### `cat` program (examples/cat.brn)

```brain
// cat program
// while condition can be an `in` statement, or valid expression of size 1 byte
// Continues so long as the given byte is not zero
while in ch[1] {
    out ch;
}
```

Compile this with `brain examples/cat.brn`.

This compiles to the following brainfuck:

```brainfuck
,[.,]
```

Run this with `brainfuck cat.bf < someinputfile.txt`.

### Reading Input (examples/input.brn)

```brain
// input requires explicit sizing
// always reads exactly this many characters or panics if EOF is reached before then
// if this many characters aren't available yet, it waits for you to send that many
in b[5];
out "b = " b "\n";

c[1] = "c";
in c;
out "c = " c "\n";

// You can reuse allocated space again
in b;
out "b = " b "\n";

// Error because we don't support dynamic length strings
//in input[];

// Error because you can't redeclare an existing name
//in b[5];

// Error because you aren't requesting any characters
//in zero[0];
```

This compiles into the following brainfuck:

```brainfuck
,>,>,>,>,>++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++.-----------
----------------------------------------
---------------.++++++++++++++++++++++++
+++++.-----------------------------.----
----------------------------<<<<<.>.>.>.
>.>++++++++++.++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++
+++++++++++++++++++++++,>+++++++++++++++
++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++
++++.-----------------------------------
--------------------------------.+++++++
++++++++++++++++++++++.-----------------
------------.---------------------------
-----<.>++++++++++.----------<<<<<<,>,>,
>,>,>>++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++.---------------
----------------------------------------
-----------.++++++++++++++++++++++++++++
+.-----------------------------.--------
------------------------<<<<<<.>.>.>.>.>
>++++++++++.----------<<<<<<
```

## Installation

For people just looking to use brain, the easiest way to get brain right now
is to first install the [Cargo package manager][cargo-install] for the
Rust programming language.

Then in your terminal run:

```
cargo install brain
cargo install brain-brainfuck
```

If you are upgrading from a previous version, run:

```
cargo install brain --force
cargo install brain-brainfuck --force
```

## Usage

**For anyone just looking to compile with the compiler:**

1. Follow the installation instructions above
2. Run `brain yourfile.brn` to compile your brain code
3. Run `brainfuck yourfile.bf` to run a brainfuck interpreter which will
   run your generated brainfuck code

You can also specify an output filename. Run `brain --help` for more information.

**For anyone looking to build the source code:**

This project contains both the brain compiler and a basic brainfuck interpreter.

Make sure you have [rust][rust] and cargo (comes with rust) installed.

### brain compiler

To compile a brain (.brn) file into brainfuck (.bf)
```
cargo run --bin brain -- filename.brn
```
where `filename.brn` is the brain program you want to compile

Use `--help` to see further options and additional information
```
cargo run --bin brain -- --help
```

**If the brain compiler seems to be taking too long or "hanging", try running
`cargo build` first to see if the rust compiler is just taking too long for
some reason.**

## Examples

There are various brain examples in the `examples/` directory which you can
compile into brainfuck using the usage instructions above.

[brainfuck]: http://www.muppetlabs.com/~breadbox/bf/
[rust]: https://www.rust-lang.org/
[cargo-install]: https://crates.io/install
