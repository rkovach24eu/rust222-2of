/// Перевіряє, чи є число `n` паліндромом.
fn is_palindromic_number(n: u32) -> bool {
    let original = n;
    let mut reversed = 0;
    let mut temp = n;

    while temp > 0 {
        reversed = reversed * 10 + temp % 10;
        temp /= 10;
    }

    original == reversed
}

// Тестові кейси
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindromic_numbers() {
        assert_eq!(is_palindromic_number(121), true);
        assert_eq!(is_palindromic_number(12321), true);
        assert_eq!(is_palindromic_number(0), true);
        assert_eq!(is_palindromic_number(1), true);
    }

    #[test]
    fn test_non_palindromic_numbers() {
        assert_eq!(is_palindromic_number(123), false);
        assert_eq!(is_palindromic_number(1234), false);
        assert_eq!(is_palindromic_number(10), false);
    }
}
