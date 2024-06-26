#![allow(dead_code)]

// function we want to test
fn sum(a: i8, b: i8) -> i8 {
    a + b
}

#[cfg(test)]
mod tests {
    fn sum_inputs_outputs() -> Vec<((i8, i8), i8)> {
        vec![((1, 1), 2), ((0, 0), 0), ((2, -2), 0)]
    }

    pub fn silly_loop() {
        for _ in 1..1_000_000_000 {}
    }

    #[test]
    fn test_sums() {
        for (input, output) in sum_inputs_outputs() {
            assert_eq!(crate::sum(input.0, input.1), output);
        }
    }

    #[test]
    fn basic_test() {
        assert!(true);
    }

    #[test]
    #[ignore]
    pub fn test_silly_loop() {
        silly_loop();
    }

    #[test]
    #[should_panic]
    fn this_panics() {
        panic!("Succeeded in failing!");
    }
}
