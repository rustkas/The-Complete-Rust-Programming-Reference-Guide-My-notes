pub fn take_the_n(n: &mut u8) {
    *n += 2;
}

pub fn take_the_s(s: &mut String) {
    s.push_str("ing");
}


#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_take_the_n() {
        let mut n = 5;
        take_the_n(&mut n);
        assert_eq!(n, 7);
    }

    #[test]
    fn test_take_the_s() {
        let mut s = String::from("Borrow");
        take_the_s(&mut s);
        assert_eq!(s, "Borrowing");
    }
}
