fn main() {
    // does not include 10
    // counts till 9
    print!("Normal ranges:    ");
    let mut string = String::new();
    for i in 0..10 {
        string += format!("{i},").as_str();
    }
    string.pop();
    print!("{string}");

    println!(); // just a newline
    string.clear();
    print!("Inclusive ranges: ");
    // counts till 10
    for i in 0..=10 {
        string += format!("{i},").as_str();
    }
    string.pop();
    println!("{string}");
    
    // optional
    string.clear();
    drop(string);
}
