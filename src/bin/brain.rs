#[macro_use]
extern crate clap;

extern crate brain;

use std::process;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use clap::{Arg, App};

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

    let source_file = Path::new(args.value_of("input-file").unwrap());
    if !source_file.exists() {
        println!("No such file: '{}'", source_file.display());
        process::exit(1);
    }

    let output = args.value_of("output-file").map_or_else(|| {
        let mut path = source_file.to_path_buf();
        path.set_extension("bf");
        path
    }, |s| PathBuf::from(s));

    println!("{} -> {}", source_file.display(), output.display());

//    println!("Source Code:\n\n{}\n", source);
//
//    let mut f = try!(File::open("foo.txt"));
//    let mut s = String::new();
//    try!(f.read_to_string(&mut s));
//    println!("AST:\n\n{:#?}", brain::parse(source));
}
