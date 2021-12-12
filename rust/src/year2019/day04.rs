use std::collections::HashMap;

pub fn run_part1() {
    let mut count = 0;
    for i in 138241..=674034 {
        if is_valid_password(i) {
            count += 1;
        }
    }

    println!("{} valid passwords", count);
}

pub fn run_part2() {
    let mut count = 0;
    for i in 138241..=674034 {
        if is_valid_strict_password(i) {
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

fn is_valid_strict_password(password: i32) -> bool {
    let digits = get_digits(password);
    let mut previous_digit = 0;
    let mut map = HashMap::new();

    for digit in digits {
        if previous_digit > digit {
            return false;
        }

        if let Some(x) = map.get_mut(&digit) {
            *x += 1;
        } else {
            map.insert(digit, 1);
        }

        previous_digit = digit;
    }

    map.values().any(|x| x == &2)
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

    #[test]
    fn test_is_valid_strict_password1() {
        assert!(is_valid_strict_password(112233));
    }

    #[test]
    fn test_is_valid_strict_password2() {
        assert!(!is_valid_strict_password(123444));
    }

    #[test]
    fn test_is_valid_strict_password3() {
        assert!(is_valid_strict_password(111122));
    }
}
