pub fn lazy_adder(a: u32, b: u32) -> impl Fn() -> u32 {
    move || a + b
}

#[cfg(test)]
mod tests {
    use super::lazy_adder;

    #[test]
    fn test_lazy_adder_basics() {
        let add_five = lazy_adder(3, 2);
        assert_eq!(5, add_five());

        let add_ten = lazy_adder(10, 0);
        assert_eq!(10, add_ten());

        let add_hundred = lazy_adder(50, 50);
        assert_eq!(100, add_hundred());
    }

    #[test]
    fn test_lazy_adder_multiple_calls() {
        let add_five = lazy_adder(3, 2);

        assert_eq!(5, add_five()); // Expected value first
        assert_eq!(5, add_five()); // Expected value first

        let add_ten = lazy_adder(10, 0);

        assert_eq!(10, add_ten()); // Expected value first
        assert_eq!(10, add_ten()); // Expected value first
    }
}
