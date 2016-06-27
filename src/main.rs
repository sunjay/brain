/// As specified here: http://www.muppetlabs.com/~breadbox/bf/

mod interpreter;

use std::env;
use std::process;

use std::io::prelude::*;
use std::fs::File;

use interpreter::interpret;

fn main() {
    let argv = env::args().collect::<Vec<_>>();
    assert!(argv.len() > 0);

    if argv.len() != 2 {
        println!("Usage: {} filename", argv[0]);
        process::exit(1);
    }

    let f = File::open(argv[1].clone()).expect("Could not open file");

    let program = f.bytes().map(
        |c| c.expect("Could not read char") as char
    ).collect::<Vec<char>>();

    interpret(program);
}
