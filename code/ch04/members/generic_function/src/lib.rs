pub fn give_me<T>(value: T) {
  let _ = value;
}



#[cfg(test)]
mod tests {
    use super::give_me;

    #[test]
    fn test_give_me_with_integer() {
        let value = 5;
        give_me(value);
    }
    
    #[test]
    fn test_give_me_with_string() {
        let value = "Hello, world!".to_string();
        give_me(value);
    }
    
    #[test]
    fn test_give_me_with_boolean() {
        let value = true;
        give_me(value);
    }

}
