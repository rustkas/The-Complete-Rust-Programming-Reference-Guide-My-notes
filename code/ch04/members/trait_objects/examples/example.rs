use trait_objects::{Area, Rectangle, Square};

fn main() {
  let shapes: Vec<&dyn Area> = vec![&Square(3f32), &Rectangle(4f32, 2f32)];
  for s in shapes {
      println!("{:?}", s);
  }
}
