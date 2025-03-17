fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    let shift = (n % len + len) % len; // Обробка негативних значень та значень, що перевищують довжину рядка
    if shift == 0 {
        return s;
    }
    let (left, right) = s.split_at((len - shift) as usize);
    right.to_string() + left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0, "abcdefgh"),
            (8, "abcdefgh"),
            (-8, "abcdefgh"),
            (1, "habcdefg"),
            (2, "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10, "cdefghab"),
        ];

        for (n, exp) in shifts.iter() {
            assert_eq!(rotate(s.clone(), *n), exp.to_string());
        }
    }
}
