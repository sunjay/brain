use std::io;
use std::io::Read;

const BUF_SIZE: usize = 30_000;

pub fn interpret(program: Vec<char>) {
    let mut buffer = [0u8; BUF_SIZE];

    let mut p: usize = 0;
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

