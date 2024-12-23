fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn you_can_assert() {
        assert!(is_even(4));
        assert!(is_even(6));
    }
}
