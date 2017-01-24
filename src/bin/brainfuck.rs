/// Brainfuck language interpreter
/// As specified here: http://www.muppetlabs.com/~breadbox/bf/
#[macro_use]
extern crate clap;

use std::process;
use std::path::{Path};
use std::io;
use std::io::prelude::*;
use std::fs::File;

use clap::{Arg, App};

const BUF_SIZE: usize = 30_000;

macro_rules! exit_with_error(
    ($($arg:tt)*) => { {
        writeln!(&mut ::std::io::stderr(), $($arg)*)
            .expect("Failed while printing to stderr");
        process::exit(1);
    } }
);

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn main() {
    let args = App::new(crate_name!())
        .version(crate_version!())
        .version_short("v")
        .author(crate_authors!())
        .about("Brainfuck interpreter companion to the brain language compiler")
        .arg(Arg::with_name("input-file")
            .help("The brainfuck file to process. Should contain brainfuck instructions")
            .value_name("file")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("debug-enabled")
            .short("D")
            .long("debug")
            .help("Enables debug mode which outputs debugging information to stderr")
        )
        .get_matches();

    let source_path = Path::new(args.value_of("input-file").unwrap());
    if !source_path.exists() || !source_path.is_file() {
        exit_with_error!("Not a valid file: '{}'", source_path.display());
    }

    let debug_mode = args.is_present("debug-enabled");

    let f = File::open(source_path).unwrap_or_else(|e| {
        exit_with_error!("Could not open source file: {}", e);
    });

    let program = f.bytes().map(
        |c| c.expect("Fatal: Could not read char") as char
    ).collect::<Vec<char>>();

    interpret(program, debug_mode);
}

fn interpret(program: Vec<char>, debug: bool) {
    if debug {
        println_stderr!("last instruction number,last instruction,current pointer,memory dump");
    }

    let mut buffer = [0u8; BUF_SIZE];

    // p is the position "pointer" in the buffer
    let mut p: usize = 0;
    // i is the instruction index in the program
    let mut i: usize = 0;

    loop {
        if i >= program.len() {
            break;
        }
        let c = program[i];
        i += 1;

        match c {
            '>' => p = (p + 1) % BUF_SIZE,
            '<' => p = (p - 1) % BUF_SIZE,
            '+' => buffer[p] = buffer[p].wrapping_add(1),
            '-' => buffer[p] = buffer[p].wrapping_sub(1),
            '.' => print!("{}", buffer[p] as char),
            ',' => {
                let chr = io::stdin().bytes().next(); if chr.is_none() {
                    break;
                }
                buffer[p] = chr.unwrap().expect("Could not read input");
            },
            '[' => {
                if buffer[p] == 0 {
                    i = find_matching(&program, i - 1) + 1;
                }
            },
            ']' => {
                if buffer[p] != 0 {
                    i = find_matching(&program, i - 1) + 1;
                }
            },
            _ => continue,
        }

        if debug {
            println_stderr!("{},{},{},{}", i-1, c, p,
                buffer.iter().fold(String::new(), |acc, v| format!("{} {}", acc, v)));
        }
    }
}

/// Finds the matching '[' or ']' for the given position within the program
/// panics if a match is not found
fn find_matching(program: &Vec<char>, start: usize) -> usize {
    let direction: isize = match program[start] {
        '[' => 1,
        ']' => -1,
        _ => unreachable!(),
    };

    let mut count = direction;
    let mut current = start;
    loop {
        if (direction < 0 && current == 0) || (direction > 0 && current >= program.len() - 1) {
            panic!("Could not find matching parenthesis for instruction {}", start);
        }
        current = (current as isize + direction) as usize;
        let c = program[current];

        count = match c {
            '[' => count + 1,
            ']' => count - 1,
            _ => count,
        };

        if count == 0 {
            break;
        }
    }

    current
}
