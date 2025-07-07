pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_magic_number(n: u64) -> u64 {
    n + 39
}

pub fn add3(n: u64) -> u64 {
    n + 3
}

pub fn add7(n: u64) -> u64 {
    n + 7
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
    fn test_add3() {
        let result = add3(2);
        assert_eq!(result, 5);
    }

    #[test]
    fn fn_subtract() {
        let result = subtract(2, 4);
        assert_eq!(result, -2);
    }
}
