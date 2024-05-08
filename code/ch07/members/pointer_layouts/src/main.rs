trait Position {}
pub struct Coordinates(pub f64, pub f64);
impl Position for Coordinates {}

fn main() {
    let val = Coordinates(1.0, 2.0);
    let ref_: &Coordinates = &val;
    let pos_ref: &dyn Position = &val as &dyn Position;
    let ptr: *const Coordinates = &val as *const Coordinates;
    let pos_ptr:  *const dyn Position = &val as * const dyn Position;

    println!("ref_: {}", std::mem::size_of_val(&ref_));
    println!("ptr: {}", std::mem::size_of_val(&ptr));
    println!("val: {}", std::mem::size_of_val(&val));
    println!("pos_ref: {}", std::mem::size_of_val(&pos_ref));
    println!("pos_ptr: {}", std::mem::size_of_val(&pos_ptr));
}
