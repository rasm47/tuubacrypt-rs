extern crate clap;
use clap::{App, Arg, ArgGroup};
use std::fs::File;
use std::io::prelude::*;
mod tuuba;

fn args() -> clap::ArgMatches<'static> {
    App::new("tuubacrypt")
        .about("Encrypt or decrypt things with the tuubacrypt algorithm")
        .arg(
            Arg::with_name("encrypt")
                .short("e")
                .long("encrypt")
                .help("Encrypt things"),
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
        .group(
            ArgGroup::with_name("data")
                .args(&["text", "file"])
                .required(true),
        )
        .get_matches()
}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}

fn write_file(filename: &str, content: &str) -> Result<(), std::io::Error> {
    File::create(filename)?.write(content.as_bytes())?;
    Ok(())
}

fn main() {
    let args = args();

    let instruction = match args.is_present("decrypt") {
        true => tuuba::Instruction::Decrypt,
        false => tuuba::Instruction::Encrypt,
    };

    let text = match args.values_of("text") {
        Some(values) => values.collect::<Vec<&str>>().join(" "),
        None => String::new(),
    };

    let filename = args.value_of("file").unwrap_or("");

    if args.is_present("file") {
        let read_result = read_file(filename);
        match read_result {
            Err(e) => {
                println!("error: {}", e);
                println!("exiting...");
                return;
            }
            Ok(content) => {
                match write_file("out.txt", &tuuba::crypt(&content, &instruction)) {
                    Err(e) => println!("error: {}", e),
                    Ok(_) => println!("file written"),
                };
            }
        };
    } else {
        // file option not given, just encrypt/decrypt the text
        println!("{}", tuuba::crypt(&text, &instruction));
    }
}
