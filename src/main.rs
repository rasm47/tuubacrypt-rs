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
                .args(&["decrypt", "encrypt"]),
        )
        .arg(
            Arg::with_name("data")
                .help("data to encrypt/decrypt")
                .required(true)
                .min_values(1),
        )
        .get_matches()
}

fn main() {
    let args = args();

    let instruction = match args.is_present("decrypt") {
        true => tuuba::Instruction::Decrypt,
        false => tuuba::Instruction::Encrypt,
    };

    let data = match args.values_of("data") {
        Some(values) => values.collect::<Vec<&str>>().join(" "),
        None => String::new(),
    };

    println!("{}", tuuba::crypt(&data, &instruction));
}
