use std::panic;

fn potentially_panicking_function() {
    panic!("Oops, something went wrong!");
}

fn main() {
    let result = panic::catch_unwind(|| {
        potentially_panicking_function();
    });

    match result {
        Ok(_) => println!("Function called successfully"),
        Err(_) => println!("Function panicked"),
    }
}
