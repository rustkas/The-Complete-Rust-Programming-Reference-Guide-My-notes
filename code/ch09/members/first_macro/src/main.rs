use first_macro::scanline;

fn main() {
    let mut input = String::new();
    println!("Enter you name: ");
    scanline!(input);
    println!("Hi {input}",);

    println!("Enter your friend's name: ");
    let a = scanline!();
    println!("Hi {a}",);
}
