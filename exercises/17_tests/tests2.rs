fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(power_of_2(11), 2048);
        assert_eq!(power_of_2(13), 8192);
        assert_eq!(power_of_2(14), 16384);
        assert_eq!(power_of_2(15), 32768);
    }
}
