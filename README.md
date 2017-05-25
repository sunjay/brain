# brain

[![Crates.io](https://img.shields.io/crates/v/brain.svg)](https://crates.io/crates/brain)
[![Crates.io](https://img.shields.io/crates/l/brain.svg)](https://crates.io/crates/brain)
[![Build Status](https://travis-ci.org/brain-lang/brain.svg?branch=master)](https://travis-ci.org/brain-lang/brain)
[![Build status](https://ci.appveyor.com/api/projects/status/hh3q7wbsna55inv6?svg=true)](https://ci.appveyor.com/project/sunjay/brain)
[![Dependency Status](https://dependencyci.com/github/brain-lang/brain/badge)](https://dependencyci.com/github/brain-lang/brain)
[![Gitter](https://img.shields.io/gitter/room/brain-lang/brain.svg)](https://gitter.im/brain-lang/brain)

brain is a strongly-typed, high-level programming language that compiles into
brainfuck. Its syntax is based on the [Rust programming language][rust] (which
it is also implemented in). Though many Rust concepts will work in brain, it
deviates when necessary in order to better suit the needs of brainfuck
programming.

[brainfuck] is an esoteric programming language with only 8 single-byte
instructions: `+`, `-`, `>`, `<`, `,`, `.`, `[`, `]`. These limited instructions
make brainfuck code extremely verbose and difficult to write. It can take a long
time to figure out what a brainfuck program is trying to do. brain makes it
easier to create brainfuck programs by allowing you to write in a more readable
and understandable language.

The type system makes it possible to detect a variety of logical errors when
compiling, instead of waiting until runtime. This is an extra layer of
convenience that brainfuck does not have. The compiler takes care of generating
all the necessary brainfuck code to work with the raw bytes in the brainfuck
turing machine.

The brain programming language compiles directly into brainfuck. The generated
brainfuck code can be run by a [brainfuck interpreter][brainfuck-interpreter].
brain only targets this interpreter which means that its generated programs are
only guaranteed to work when run with that. The interpreter implements a
[brainfuck specification][bf-spec] specially designed and written for the brain
programming language project.

## Optimization Goals

The brain compiler is designed to optimize the generated brainfuck code as much
as possible.

1. Generate small brainfuck files (use as few instructions as possible)
2. Generate memory efficient code (use as few brainfuck cells as possible)

Optimization is an ongoing effort. As the project matures, these goals will
become more expressed in the compiled output of the program.

## brain syntax

For full examples, please see the `examples/` directory. Some examples aren't
fully implemented yet in the compiler.

### `cat` program (examples/cat.brn)

```rust
// cat program
let mut ch: [u8; 1];

while true {
  // stdin.read_exact() panics if EOF is reached
  stdin.read_exact(ch);
  stdout.print(ch);
}
```

Compile this with `brain examples/cat.brn`.

Run this with `brainfuck cat.bf < someinputfile.txt`.

### Reading Input (examples/input.brn)

```rust
// input requires explicit sizing
// always reads exactly this many characters or panics if EOF is reached before then
// if this many characters aren't available yet, it waits for you to send that many
let mut b: [u8; 5];
stdin.read_exact(b);
stdout.print(b"b = ", b, b"\n");

let mut c: [u8; 1];
stdin.read_exact(c);
stdout.print(b"c = ", c, b"\n");

// You can reuse allocated space again
stdin.read_exact(b);
stdout.print(b"b = ", b, b"\n");
```

Compile this with `brain examples/input.brn`.

This compiles into the following brainfuck:

```brainfuck
,>,>,>,>,>+++++++++++++++++++++++++++++++++
+++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++.--------------------
-------------------------------------------
---.+++++++++++++++++++++++++++++.---------
--------------------.----------------------
----------<<<<<.>.>.>.>.>++++++++++.-------
---,>++++++++++++++++++++++++++++++++++++++
+++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++.------------------------
-------------------------------------------
.+++++++++++++++++++++++++++++.------------
-----------------.-------------------------
-------<.>++++++++++.----------<<<<<<,>,>,>
,>,>>++++++++++++++++++++++++++++++++++++++
+++++++++++++++++++++++++++++++++++++++++++
+++++++++++++++++.-------------------------
-----------------------------------------.+
++++++++++++++++++++++++++++.--------------
---------------.---------------------------
-----<<<<<<.>.>.>.>.>>++++++++++.----------
```

Run this after compiling with `brainfuck input.bf < someinputfile.txt`.

## Installation

For people just looking to use brain, the easiest way to get brain right now
is to first install the [Cargo package manager][cargo-install] for the
Rust programming language.

**NOTE: Until this version is released, these instructions will NOT work. Please
see the Usage instructions below for how to manually install the compiler from the
source code.**

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

Make sure you have [Rust][rust] and cargo (comes with Rust) installed.

### brain compiler

To compile a brain (.brn) file into brainfuck (.bf)
```
cargo run filename.brn
```
where `filename.brn` is the brain program you want to compile

Use `--help` to see further options and additional information
```
cargo run -- --help
```

**If the brain compiler seems to be taking too long or "hanging", try running
`cargo build` first to see if the Rust compiler is just taking too long for
some reason.**

You can also install the compiler from the source code using this command in the
repository's root directory:

```
cargo install --path .
```

## Examples

There are various brain examples in the `examples/` directory which you can
compile into brainfuck using the usage instructions above.

## Thanks

This project would not be possible without the brilliant work of the many
authors of the [Esolang Brainfuck Algorithms][bf-algorithms] page. The entire
wiki has been invaluable. That page in particular is the basis for a lot of the
code generation in this compiler. I have contributed many novel brainfuck
algorithms to that page as I come up with them for use in this compiler.

[brainfuck]: http://www.muppetlabs.com/~breadbox/bf/
[rust]: https://www.rust-lang.org/
[cargo-install]: https://crates.io/install
[bf-algorithms]: https://esolangs.org/wiki/Brainfuck_algorithms
[brainfuck-interpreter]: https://github.com/brain-lang/brainfuck
[bf-spec]: https://github.com/brain-lang/brainfuck/blob/master/brainfuck.md
