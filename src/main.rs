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
    if upper_limit <= lower_limit || c < lower_limit || c > upper_limit {
        return Err("Invalid input");
    }

    let modulo = 1 + upper_limit as i32 - lower_limit as i32;
    let shift = (modulo + rotation % modulo) % modulo;
    let original_position = c as i32 - lower_limit as i32;
    let rotated_position = (original_position + shift) % modulo;

    Ok((lower_limit as u8 + rotated_position as u8) as char)
}

fn rotate_digit(digit: char, rotations: i32) -> char {
    bounded_rotate(digit, rotations, '0', '9').unwrap_or(digit)
}

fn rotate_upper(uppercase_letter: char, rotations: i32) -> char {
    bounded_rotate(uppercase_letter, rotations, 'A', 'Z').unwrap_or(uppercase_letter)
}

fn encrypt(data: &String) -> String {
    let encrypt_char = |(i, c): (_, char)| {
        if c.is_ascii_digit() {
            rotate_digit(c, 1 + i as i32)
        } else if c.is_ascii_uppercase() {
            rotate_upper(c, 1 + i as i32)
        } else {
            c
        }
    };

    data.chars()
        .enumerate()
        .map(encrypt_char)
        .collect()
}

fn main() {
    let args = args();

    let decrypt_flag = args.is_present("decrypt");

    let data = match args.values_of("data") {
        Some(values) => values.collect::<Vec<&str>>().join(" "),
        None => String::new(),
    };

    if decrypt_flag {
        println!("todo decryption");
    } else {
        println!("after encrypiton: {}", encrypt(&data));
    }

    println!("{}", data);
}
