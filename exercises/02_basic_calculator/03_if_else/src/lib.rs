/// Return `true` if `n` is even, `false` otherwise.
fn is_even(n: u32) -> bool {
    n%2u32 == 0
}

#[cfg(test)]
mod tests {
    use crate::is_even;

    #[test]
    fn one() {
        assert!(!is_even(1));
    }

    #[test]
    fn two() {
        assert!(is_even(2));
    }

    #[test]
    fn high() {
        assert!(!is_even(231));
    }
}
