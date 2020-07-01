pub fn run_part1() {
    let mut count = 0;
    for i in 138241..674034 {
        if is_valid_password(i) {
            count += 1;
        }
    }

    println!("{} valid passwords", count);
}

fn is_valid_password(password: i32) -> bool {
    let digits = get_digits(password);
    let mut adjacent_digits_match = false;
    let mut previous_digit = 0;

    for digit in digits {
        if previous_digit > digit {
            return false;
        }

        if previous_digit == digit {
            adjacent_digits_match = true;
        }

        previous_digit = digit;
    }

    adjacent_digits_match
}

fn get_digits(password: i32) -> Vec<i32> {
    let mut components = Vec::new();

    let mut remaining = password;

    loop {
        components.push(remaining % 10);
        remaining = remaining / 10;

        if remaining <= 0 {
            break;
        }
    }

    components.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_password1() {
        assert!(is_valid_password(111111));
    }

    #[test]
    fn test_is_valid_password2() {
        assert!(!is_valid_password(223450));
    }

    #[test]
    fn test_is_valid_password3() {
        assert!(!is_valid_password(123789));
    }
}
