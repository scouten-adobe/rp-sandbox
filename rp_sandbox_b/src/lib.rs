use rp_sandbox_a::add3;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add6(n: u64) -> u64 {
    add3(add3(n))
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
    fn test_add6() {
        let result = add6(2);
        assert_eq!(result, 8);
    }
}
