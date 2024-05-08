use std::thread;

fn main() {
    let my_str = String::from("Damn you borrow checker!");
    let cloned_str = my_str.clone();
    let _ = thread::spawn(move || {
        println!("In thread: {cloned_str}");
    });

    println!("In main: {}", my_str);
}
