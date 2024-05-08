#[cfg(test)]
mod tests {
    // use super::*;

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_result_contains_value() {
            let my_result: Result<_, ()> = Ok(64);
            assert_eq!(my_result, Ok(64));
        }

        #[test]
        fn test_result_contains_value02() {
            let my_result = Ok::<_, ()>(64);
            assert_eq!(my_result, Ok(64));
        }

        #[test]
        fn test_result_contains_error() {
            let my_err = Err::<(), f32>(345.3);
            assert_eq!(my_err, Err(345.3));
        }

        #[test]
        fn test_result_contains_error02() {
            let other_err: Result<bool, String> = Err("Wait, what ?".to_string());
            assert_eq!(other_err, Err("Wait, what ?".to_string()));
        }
    }
}
