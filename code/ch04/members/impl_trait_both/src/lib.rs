use std::fmt::Display;

pub fn surround_with_braces(val: impl Display) -> impl Display {
    format!("{{{val}}}")
}


#[cfg(test)]
mod tests {
    use super::surround_with_braces;

    #[test]
    fn test_surround_with_integers() {
        let result = surround_with_braces(5);
        assert_eq!(format!("{}", result), "{5}");

        let result = surround_with_braces(-10);
        assert_eq!(format!("{}", result), "{-10}");
    }

    #[test]
    fn test_surround_with_strings() {
        let result = surround_with_braces("Hello, world!");
        assert_eq!(format!("{}", result), "{Hello, world!}");
        let result = surround_with_braces("Hello, world!".to_string());
        assert_eq!(format!("{}", result), "{Hello, world!}");
        
    }

    #[test]
    fn test_surround_with_floats() {
        let result = surround_with_braces(3.14159);
        assert_eq!(format!("{}", result), "{3.14159}");

        let result = surround_with_braces(-22);
        assert_eq!(format!("{}", result), "{-22}");
    }
}