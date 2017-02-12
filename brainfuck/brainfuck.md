# Brainfuck Specification

| Key | Value |
| --- | --- | --- |
| Version | 1.0.0 |
| Author | Sunjay Varma (sunjay.ca) |
| License | GNU General Public License (Version 3)<br>https://www.gnu.org/licenses/gpl-3.0.en.html<br>This license applies to the specification and not code written using it. |

## Background

*Adapted from the [Wikipedia article on Brainfuck][brainfuck-wiki].*

Brainfuck is an esoteric programming language created in 1993 by Urban Müller.
The language contains only eight simple commands and an instruction pointer.
While it is fully Turing-complete, it is not intended for practical use, but to
challenge and amuse programmers.

The language's name is a reference to the slang term brainfuck, which refers to
things so complicated or unusual that they exceed the limits of one's
understanding.

## Scope and Justification

When the language was first conceived of, the creator specified its [eight
instructions][bf-instructions], but not much else about how the language should
behave at runtime. This led to a lot of ambiguity and many varying
implementations. This document intends to resolve those issues and specify
the behaviour in as much detail as possible.

This document proposes just *one* way that the brainfuck programming language
could behave. There are many implementations of brainfuck. **This is not *the*
brainfuck specification.** Each implementation works differently to suit the
needs of its author. The cell sizes differ, the memory layout changes and
sometimes people even add or remove instructions from the original set. This
specification aims to tie together most of the common implementations and
assumptions into a single document which anyone can use to implement brainfuck.

While most details are specified in their entirety, some aspects are left up
to the implementer. These are mentioned explicitly throughout the document.

## Overview

Brainfuck is a turing-complete language modelled after the theoretical model of a
[turing machine][turing-machine]. As such, when programming in brainfuck, it is
good to keep a mental model of a brainfuck turing machine in mind.

The brainfuck turning machine:
```
----------------------||---||----------------------
  | 0 | 0 | 0 | 0 | 0 || 0 || 0 | 0 | 0 | 0 | 0 |
----------------------||---||----------------------
  tape ---^     ^        ^
    cell --------        |
       head (pointer) ----
```

The "tape" shown above is the memory of the computer. The "cells" in the tape
contain values. The currently selected cell is at the "head" and is sometimes
called the "pointer".

Instructions to manipulate the tape are fed into the machine. They are not
stored in the tape itself. The instructions specify how the machine should move
the tape. The cell under the head can change and the value of that cell can be
updated, replaced or outputted. The full instruction set is described in detail
in the [Instructions](#instructions) section.

## Instructions

Brainfuck has 8 basic instructions:

* `>` - move the pointer right
* `<` - move the pointer left
* `+` - increment the current cell
* `-` - decrement the current cell
* `.` - output the value of the current cell
* `,` - **replace** the value of the current cell with input
* `[` - jump to the **matching** `]` instruction if the current value is zero
* `]` - jump to the **matching** `[` instruction if the current value is **not** zero

These are specified in more detail in the sections below.

## Memory Layout

The brainfuck tape is made of an "infinite" collection of 1 byte cells. Each
cell represents a single, unsigned 8-bit number. Cells start initialized at
zero.

Since the numbers are unsigned, there is no need for any complex integer
implementation. If the upper limit of the cell is reached, it wraps back to
zero. If zero is decremented, it must wrap back to 11111111. Normal binary
number arithmetic rules applies.

### Arithmetic and Wrapping Behaviour Examples

Increment:
```
Current value: 00000011
Instruction: +
Next value: 00000100

Current value: 11111110
Instruction: +
Next value: 11111111

Current value: 11111111
Instruction: +
Next value: 00000000
```

Decrement:
```
Current value: 00000010
Instruction: -
Next value: 00000001

Current value: 00000001
Instruction: -
Next value: 00000000

Current value: 00000000
Instruction: -
Next value: 11111111
```

## The Program Counter and Address Pointer

**The Program Counter (PC)** indicates where the processor is in its program.
The majority of the time, this value will be incremented by one after every
instruction. The two exceptions to this are the two jump instructions which
cause the PC to change based on the value of the current cell indicated by the
address pointer. The program begins at the first instruction. The processor
stops running when it is out of instructions to run.

**The Address Pointer (The "Pointer")** indicates the "address" of the current
cell in memory.

For a turing machine to really be capable of modelling everything a computer can
do, the tape must be infinite. Ensure that the tape can grow in either direction
regardless of the current position. It should be possible to move left at the
"beginning" of the tape and "right" at the end.

> Implementation Note: When implemented in software, this is usually just
> an index into an array used to store the memory of the program.

For practical reasons, a truly infinite tape is not usually possible. If the
implementation is limited to a fixed amount of memory, it is appropriate to wrap
the address pointer to the start or the end of the memory addresses when a
movement is requested past one of the boundaries of the addresses. For example,
if a left instruction is used when at the first memory address, go to the last
one. If a right instruction is used, go to the first address. **This should be
clearly documented as this can cause major issues.**

It is *highly* recommended, to report an error and then abort if wrapping
results in using a cell that was already used previously for something else. For
example, if the program uses every memory address, then wraps back to the start,
the program could accidentally overwrite memory that was already used for
something else. In this situation, it is best to report some sort of memory
error and abort further execution entirely.

Note that in some situations assuming wrapping behaviour is useful because it
allows you to quickly get to either end of the memory addresses. However, since
we are using the assumption that the tape is infinite, this convenience cannot
be relied on and **should not** be used.

## Move Right (>)

Moves the pointer to the next cell (to the right of the current cell). It may
be necessary to expand the memory buffer in order to make sure
the tape is infinite.

Wrapping is not recommended. It is better to abort if previously used cells are
going to be overwritten.

**Seriously, do not overwrite cells that were previously used.** That means that when you
reach the end of your available memory, you should **not** loop back and start
overwriting the cells from the beginning.

## Move Left (<)

Moves the pointer to the previous cell (to the left of the current cell).

This instruction is almost identical in implementation to the move right
instruction. See the description of the [Move Right](#move-right-)
instruction for more details.

## Increment (+)

Increments the value of the current cell by 1. Wrap the value back to zero if
the value overflows the byte. See the [Memory Layout](#memory-layout) section
for more information about cell sizes and arithmetic.

## Decrement (-)

Decrements the value of the current cell by 1. Wrap the value back to the
maximum if the value goes below zero. See the [Memory Layout](#memory-layout)
section for more information about cell sizes and arithmetic.

## Write (.)

Writes (outputs) the value of the current cell. The specific implementation of
this command is left up to the implementer.

Some considerations:
* Typically the output device is a display or shell
* The value of a cell is represented as a plain byte which does not necessarily
  translate directly to a non-ascii character
* For full UTF-8 compatibility, it may be necessary to temporarily buffer output
  and combine the bytes into characters which can then be outputted
* Output is not limited to text, the bytes can be anything
  * Imagine implementing some 8-bit drawing commands and writing brainfuck to
    create images

## Read (,)

Reads the **next** byte from an input stream and **replaces** the value of the
current cell with that new value. The implementation and representation of the
input stream is left up to the implementer. All that is necessary is that the
stream produces **single bytes** and that the cell value is **replaced** with
that new value.

If there is no more input to read, the cell should be set to zero in order to
signify the End-of-File (EOF). This gives the program a chance to respond to
the EOF.

## Jump If Zero ([)

Jumps to the **matching** `]` instruction if the value of the current cell is
zero. If the value of the current cell is not zero, the program moves on as
normal. This has the effect of entering a "loop" body when there is a non-zero
value in the current cell. By jumping if the value is zero, some instructions
can be skipped based on the value of the current cell.

This is one of two instructions that can modify the PC.

It is important to jump to the *matching* `]` instruction so that these jumps
can be nested when necessary. If a matching `]` is not found, the program
should abort with an error message.

Example:

```brainfuck
+ [ > + [ . ] ]
1 2 3 4 5 6 7 8
```

1. Add one to the current cell
2. Jump to instruction 8 if the current cell is zero
3. Move one cell to the right
4. Add one to the current cell
5. Jump to instruction 7 if the current cell is zero
6. Output the value of the current cell
7. Jump to instruction 5 if the current cell is **not** zero
7. Jump to instruction 2 if the current cell is **not** zero

## Jump Unless Zero (])

Jumps to the **matching** `[` instruction if the value of the current cell
is **not** zero. This has the effect of jumping back to the beginning of a
"loop" while the current cell is non-zero. If the current cell is zero, the
program continues past this instruction without doing anything.

This is the second of two instructions that can modify the PC.

It is important to jump to the *matching* `[` instruction so that these jumps
can be nested when necessary. If a matching `[` is not found, the program
should abort with an error message.

See the [Jump If Zero](#jump-if-zero-) section for more information and an
example.

## Other Characters

In general, other characters found in a brainfuck file should just be ignored.
Those characters could be documentation, or something else entirely.

## Hello World Example

*Adapted from the [Wikipedia article on Brainfuck][brainfuck-wiki].*

The following program prints "Hello World!" and a newline to the screen:

```brainfuck
[ This program prints "Hello World!" and a newline to the screen, its
  length is 106 active command characters. [It is not the shortest.]

  This loop is an "initial comment loop", a simple way of adding a comment
  to a BF program such that you don't have to worry about any command
  characters. Any ".", ",", "+", "-", "<" and ">" characters are simply
  ignored, the "[" and "]" characters just have to be balanced. This
  loop and the commands it contains are ignored because the current cell
  defaults to a value of 0; the 0 value causes this loop to be skipped.
]
++++++++               Set Cell #0 to 8
[
    >++++               Add 4 to Cell #1; this will always set Cell #1 to 4
    [                   as the cell will be cleared by the loop
        >++             Add 2 to Cell #2
        >+++            Add 3 to Cell #3
        >+++            Add 3 to Cell #4
        >+              Add 1 to Cell #5
        <<<<-           Decrement the loop counter in Cell #1
    ]                   Loop till Cell #1 is zero; number of iterations is 4
    >+                  Add 1 to Cell #2
    >+                  Add 1 to Cell #3
    >-                  Subtract 1 from Cell #4
    >>+                 Add 1 to Cell #6
    [<]                 Move back to the first zero cell you find; this will
                        be Cell #1 which was cleared by the previous loop
    <-                  Decrement the loop Counter in Cell #0
]                       Loop till Cell #0 is zero; number of iterations is 8

The result of this is:
Cell No :   0   1   2   3   4   5   6
Contents:   0   0  72 104  88  32   8
Pointer :   ^

>>.                     Cell #2 has value 72 which is 'H'
>---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
+++++++..+++.           Likewise for 'llo' from Cell #3
>>.                     Cell #5 is 32 for the space
<-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
<.                      Cell #3 was set to 'o' from the end of 'Hello'
+++.------.--------.    Cell #3 for 'rl' and 'd'
>>+.                    Add 1 to Cell #5 gives us an exclamation point
>++.                    And finally a newline from Cell #6
```

For "readability", this code has been spread across many lines and blanks and
comments have been added. Brainfuck ignores all characters except the eight
commands `+-<>[],.` so no special syntax for comments is needed (as long as the
comments don't contain the command characters). The code could just as well have
been written as:

```brainfuck
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

[brainfuck-wiki]: https://en.wikipedia.org/wiki/Brainfuck
[bf-instructions]: http://www.muppetlabs.com/~breadbox/bf/
[turing-machine]: https://en.wikipedia.org/wiki/Turing_machine
