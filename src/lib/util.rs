use std::io;
use std::num::ParseIntError;

pub(crate) fn try_read_int() -> Result<u8, ParseIntError> {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let x = input_text.trim().parse::<u8>();
    return x;
}

pub(crate) fn read_int() -> u8 {
    let mut maybe_int = try_read_int();
    while maybe_int.is_err() {
        println!("Invalid input. Enter a new coordinate:");
        maybe_int = try_read_int();
    }
    return maybe_int.unwrap();
}