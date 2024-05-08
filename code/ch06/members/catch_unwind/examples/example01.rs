use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        panic!("Oops, something went wrong!");
    });

    match result {
        Ok(_) => println!("No panic occurred"),
        Err(_) => println!("Panic caught and handled"),
    }
}