use generic_struct_impl::Container;

fn main() {
  let container = Container::new("Hello, world!".to_string());
  println!("Container with string item: {}", container);
}