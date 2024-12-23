fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day < 22 {
        Some(5)
    } else if hour_of_day <= 23 {
        return Some(0);
    } else {
        None
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12).unwrap();

        assert_eq!(icecreams, 5);
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}
