use function_mut::increase_by;


fn main() {
    let score = 2048;
    println!("Current value = {score}");
    
    increase_by(score, 30);
    
    println!("After increasing value = {score}")
  }
  