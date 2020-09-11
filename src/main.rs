extern crate clap;
use clap::{App, Arg, ArgGroup};
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
            ArgGroup::with_name("gr")
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
            ArgGroup::with_name("gr2")
                .args(&["text", "file"])
                .required(true),
        )
        .get_matches()
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

    println!("{}", tuuba::crypt(&text, &instruction));
}
