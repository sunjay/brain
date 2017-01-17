#[macro_use]
extern crate clap;

extern crate brain;

use std::process;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use clap::{Arg, App};

use brain::{Program, Instructions};

macro_rules! exit_with_error(
    ($($arg:tt)*) => { {
        writeln!(&mut ::std::io::stderr(), $($arg)*)
            .expect("Failed while printing to stderr");
        process::exit(1);
    } }
);

fn main() {
    let args = App::new(crate_name!())
        .version(crate_version!())
        .version_short("v")
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("input-file")
            .help("The brain file to process")
            .value_name("file")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("output-file")
            .short("o")
            .help("Write output to <target>")
            .value_name("target")
            .takes_value(true)
        )
        .get_matches();

    let source_path = Path::new(args.value_of("input-file").unwrap());
    if !source_path.exists() || !source_path.is_file() {
        exit_with_error!("Not a valid file: '{}'", source_path.display());
    }

    let output_path = args.value_of("output-file").map_or_else(|| {
        let mut path = PathBuf::from(source_path.file_name().and_then(|s| s.to_str()).unwrap_or(""));
        path.set_extension("bf");
        path
    }, |s| PathBuf::from(s));

    let mut f = File::open(source_path).unwrap_or_else(|e| {
        exit_with_error!("Could not open source file: {}", e);
    });
    let mut source = String::new();
    f.read_to_string(&mut source).unwrap_or_else(|e| {
        exit_with_error!("Could not read source file: {}", e);
    });
    println!("Source Code:\n\n{}\n", source);

    let program: Program = source.parse().unwrap();
    println!("AST:\n\n{:#?}\n", program);

    let instructions = Instructions::from_program(program).unwrap();
    println!("Instructions:\n\n{:#?}\n", instructions);

    let generated_code: String = instructions.into();
    println!("Generated:\n{}\n", generated_code);
}
