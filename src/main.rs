extern crate clap;
use clap::{App, Arg};

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
                .min_values(1)
        )
        .get_matches()
}

fn bounded_rotate(c: char, rotation: i32, lower_limit: char, upper_limit: char) -> Result<char, &'static str> {
    if upper_limit <= lower_limit || c < lower_limit || c > lower_limit {
        return Err("Invalid input");
    }

    let modulo = 1 + upper_limit as i32 - lower_limit as i32;
    let shift = rotation % modulo;
    let shifted = (modulo + shift) % modulo;
    let rotated = lower_limit as u8 + shifted as u8;

    Ok(rotated as char)
}

fn rotate_digit(digit: char, rotations: i32) -> char {
    bounded_rotate(digit, rotations, '1', '0').unwrap_or(digit)
}

fn rotate_upper(uppercase_letter: char, rotations: i32) -> char {
    bounded_rotate(uppercase_letter, rotations, 'A', 'Z').unwrap_or(uppercase_letter)
}

fn main() {
    let args = args();

    let decrypt = args.is_present("decrypt");

    let data = match args.values_of("data") {
        Some(values) => values.collect::<Vec<&str>>().join(" "),
        None => String::new(),
    };

    if decrypt {
        println!("todo encrypt");
    } else {
        println!("todo encrypt");
    }

    println!("{}", data);
}
