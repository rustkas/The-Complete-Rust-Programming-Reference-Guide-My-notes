use impl_trait_closure::lazy_adder;

fn main() {
  let add_five = lazy_adder(3, 2); // Create a closure that adds 3 and 2

  // Use the closure to calculate sums
  let sum1 = add_five();
  println!("3 + 2 = {}", sum1); // Output: 3 + 2 = 5

  let sum2 = add_five(); // Call the closure again
  println!("3 + 2 = {}", sum2); // Output: 3 + 2 = 5

  // The closure remembers the values from its creation, so it always adds 3 and 2
}