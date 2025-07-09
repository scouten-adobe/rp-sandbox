use rp_sandbox_a::{add3, add_magic_number};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add6(n: u64) -> u64 {
    add3(add3(n))
}

pub fn add9(n: u64) -> u64 {
    add3(add6(n))
}

pub fn add_more_magic(n: u64) -> u64 {
    add_magic_number(n) + 13
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

    #[test]
    fn test_add9() {
        let result = add9(1);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_add_more_magic() {
        let result = add_more_magic(7);
        assert_eq!(result, 62);
    }
}
// Updated to use rp_sandbox_a 
// Updated to use rp_sandbox_a 
// Updated to use rp_sandbox_a 
// Updated to use rp_sandbox_a 
// Updated to use rp_sandbox_a 
// Updated to use rp_sandbox_a 
// Updated to use rp_sandbox_a 
// Updated to use rp_sandbox_a 
