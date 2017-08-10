#[macro_use]
extern crate clap;

extern crate brain;

use std::process;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use clap::{Arg, App};

use brain::parser::{Program, ParseError};
use brain::operations::scope::ScopeStack;
use brain::codegen::Instructions;
use brain::optimizations::{OptimizationLevel, Optimize};
use brain::prelude;

macro_rules! exit_with_error(
    ($($arg:tt)*) => { {
        eprintln!($($arg)*);
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

    let mut source_file = File::open(source_path).unwrap_or_else(|e| {
        exit_with_error!("Could not open source file: {}", e);
    });
    let mut source = String::new();
    source_file.read_to_string(&mut source).unwrap_or_else(|e| {
        exit_with_error!("Could not read source file: {}", e);
    });

    let generated_code: String = compile(source);
    println!("{:?}", generated_code);

    let mut output_file = File::create(output_path).unwrap_or_else(|e| {
        exit_with_error!("Could not create target file: {}", e);
    });
    output_file.write_all(generated_code.as_bytes()).and_then(|_| {
        // Write a newline because that's how a line is defined
        // http://stackoverflow.com/a/729795/551904
        output_file.write(&['\n' as u8])
    }).unwrap_or_else(|e| {
        exit_with_error!("Could not write target file: {}", e);
    });
}

/// Compile the provided source code to brainfuck
fn compile(source: String) -> String {
    let program: Program = source.parse().unwrap_or_else(|e: ParseError| {
        exit_with_error!("Syntax Error: {}", e);
    });

    let mut global_scope = ScopeStack::new();
    prelude::populate_scope(&mut global_scope);
    let operations = program.into_operations(&mut global_scope).unwrap_or_else(|error| {
        println!("{:?}", error);
        // Print the error properly
        unimplemented!();
    });
    println!("{:#?}", operations);

    let instructions: Instructions = operations.into();

    instructions.optimize(OptimizationLevel::L1).into()
}
