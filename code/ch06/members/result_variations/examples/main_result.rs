fn main() -> Result<(), String> {
  let s = vec!["apple", "mango", "banana"];
  let fourth = match s.get(3) {
      Some(&fourth) => fourth,
      None => return Err("I got only 3 fruits".to_string()),
  };
  println!("{}", fourth);
  Ok(())
}
