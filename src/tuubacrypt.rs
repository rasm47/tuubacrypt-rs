fn bounded_rotate(
    c: char,
    rotation: i32,
    lower_limit: char,
    upper_limit: char,
) -> Result<char, &'static str> {
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

pub enum TuubaInstruction {
    Encrypt,
    Decrypt,
}

pub fn tuubacrypt(data: &str, instruction: &TuubaInstruction) -> String {
    let mut rotations = 0;
    let direction = match instruction {
        TuubaInstruction::Encrypt => 1,
        TuubaInstruction::Decrypt => -1,
    };

    let tuubacrypt_char = |c: char| {
        if c.is_ascii_digit() {
            rotations += 1;
            rotate_digit(c, direction * rotations)
        } else if c.is_ascii_uppercase() {
            rotations += 1;
            rotate_upper(c, direction * rotations)
        } else {
            c
        }
    };

    data.chars().map(tuubacrypt_char).collect()
}
