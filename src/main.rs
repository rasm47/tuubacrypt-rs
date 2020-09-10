extern crate clap;
use clap::{App, Arg};
mod tuubacrypt;

fn args() -> clap::ArgMatches<'static> {
    App::new("tuubacrypt")
        .about("Encrypt or decrypt things with the tuubacrypt algorithm")
        .arg(
            Arg::with_name("decrypt")
                .short("d")
                .long("decrypt")
                .help("Decrypt instead of encypting"),
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
        true => tuubacrypt::TuubaInstruction::Decrypt,
        false => tuubacrypt::TuubaInstruction::Encrypt,
    };

    let data = match args.values_of("data") {
        Some(values) => values.collect::<Vec<&str>>().join(" "),
        None => String::new(),
    };

    println!("{}", tuubacrypt::tuubacrypt(&data, &instruction));
}
