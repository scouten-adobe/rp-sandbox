pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn subtract(left: i64, right: i64) -> i64 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn fn_subtract() {
        let result = subtract(2, 4);
        assert_eq!(result, -2);
    }
}
