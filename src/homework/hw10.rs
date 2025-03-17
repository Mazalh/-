fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        for (n, exp) in data.iter() {
            assert_eq!(is_palindrome(*n), *exp);
        }
    }
}
