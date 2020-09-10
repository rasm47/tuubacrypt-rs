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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_rotate_basic() {
        let original = 'A';
        let expected = 'B';
        let rotated = bounded_rotate(original, 1, 'A', 'Z').unwrap_or('?');
        assert_eq!(expected, rotated);
    }

    #[test]
    fn bounded_rotate_reverse() {
        let original = 'A';
        let expected = 'Z';
        let rotated = bounded_rotate(original, -1, 'A', 'Z').unwrap_or('?');
        assert_eq!(expected, rotated);
    }

    #[test]
    fn bounded_rotate_rotate_lots() {
        let original = 'A';
        let expected = 'A';
        let rotated = bounded_rotate(original, 26 * 1000, 'A', 'Z').unwrap_or('?');
        assert_eq!(expected, rotated);
    }

    #[test]
    fn bounded_rotate_bad_bounds() {
        let original = 'A';
        let expected = "Invalid input";
        let rotated = bounded_rotate(original, 1, 'Z', 'A').unwrap_err();
        assert_eq!(expected, rotated);
    }

    #[test]
    fn bounded_rotate_c_out_of_bounds() {
        let original = '8';
        let expected = "Invalid input";
        let rotated = bounded_rotate(original, 1, '1', '7').unwrap_err();
        assert_eq!(expected, rotated);
    }

    #[test]
    fn rotate_digit_basic() {
        let original = '3';
        let expected = '6';
        let rotated = rotate_digit(original, 3);
        assert_eq!(expected, rotated);
    }

    #[test]
    fn rotate_digit_bad_digit() {
        let original = 'F';
        let expected = 'F';
        let rotated = rotate_digit(original, 3);
        assert_eq!(expected, rotated);
    }

    #[test]
    fn rotate_upper_basic() {
        let original = 'C';
        let expected = 'G';
        let rotated = rotate_upper(original, 4);
        assert_eq!(expected, rotated);
    }

    #[test]
    fn rotate_upper_bad_letter() {
        let original = '?';
        let expected = '?';
        let rotated = rotate_upper(original, 3);
        assert_eq!(expected, rotated);
    }

    #[test]
    fn tuubacrypt_encrypt() {
        let original = "AAAaaa000";
        let expected = "BCDaaa456";
        let rotated = tuubacrypt(original, &TuubaInstruction::Encrypt);
        assert_eq!(expected, rotated);
    }

    #[test]
    fn tuubacrypt_decrypt() {
        let original = "hjk555eeeRRR";
        let expected = "hjk432eeeNML";
        let rotated = tuubacrypt(original, &TuubaInstruction::Decrypt);
        assert_eq!(expected, rotated);
    }
}
