extern crate clap;
use clap::{App, Arg, ArgGroup};
use std::fs::File;
use std::io::prelude::*;
mod tuuba;

const DEFAULT_OUTPUT_FILENAME: &str = "out.txt";

fn args() -> clap::ArgMatches<'static> {
    App::new("tuubacrypt")
        .about("Encrypt or decrypt things with the tuubacrypt cipher")
        .arg(
            Arg::with_name("encrypt")
                .short("e")
                .long("encrypt")
                .help("Encrypt things (default)"),
        )
        .arg(
            Arg::with_name("decrypt")
                .short("d")
                .long("decrypt")
                .help("Decrypt things"),
        )
        .group(
            ArgGroup::with_name("flags")
                .args(&["decrypt", "encrypt"])
                .required(false),
        )
        .arg(
            Arg::with_name("text")
                .help("text to encrypt/decrypt")
                .min_values(1),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("File to encrypt/decrypt")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Output filename e.g. foo.txt")
                .requires("file")
                .default_value(DEFAULT_OUTPUT_FILENAME)
                .takes_value(true),
        )
        .group(
            ArgGroup::with_name("data")
                .args(&["file", "text"])
                .required(true),
        )
        .get_matches()
}

fn cipher_file(
    input_filepath: &str,
    output_filename: &str,
    instruction: &tuuba::Instruction,
) -> Result<(), std::io::Error> {
    let mut content = String::new();
    File::open(input_filepath)?.read_to_string(&mut content)?;

    let tuubacrypted_content = &tuuba::cipher(&content, &instruction);
    File::create(output_filename)?.write_all(tuubacrypted_content.as_bytes())?;
    Ok(())
}

fn main() {
    let args = args();

    let instruction = match args.is_present("decrypt") {
        true => tuuba::Instruction::Decrypt,
        false => tuuba::Instruction::Encrypt,
    };

    if args.is_present("file") {
        let filename = args.value_of("file").unwrap_or("");
        let output = args.value_of("output").unwrap_or(DEFAULT_OUTPUT_FILENAME);

        match cipher_file(filename, &output, &instruction) {
            Err(e) => println!("Err: {}", e),
            Ok(_) => println!("Done!"),
        }
        return;
    }

    // file option not given, just cipher the text
    let text = match args.values_of("text") {
        Some(values) => values.collect::<Vec<&str>>().join(" "),
        None => String::new(),
    };

    print!("{}", tuuba::cipher(&text, &instruction));
}
