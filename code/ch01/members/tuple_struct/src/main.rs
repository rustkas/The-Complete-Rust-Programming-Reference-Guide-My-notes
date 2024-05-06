use tuple_struct::Color;

fn main() {
    let white = Color(255, 255, 255);
    
    // You can pull them out by index
    let mut red = white.0;
    let green = white.1;
    let mut blue = white.2;

    println!("Red value: {}", red);
    println!("Green value: {}", green);
    println!("Blue value: {}\n", blue);

    let orange = Color(255, 165, 0);

    // You can also destructure the fields directly
    let Color(r, g, b) = orange;
    println!("R: {}, G: {}, B: {} (orange)", r, g, b);

    // Can also ignore fields while destructuring
    let Color(r, _, b) = orange;
    red = r;
    blue = b;
    println!("Red value: {}", red);
    println!("Green value: {}", green);
    println!("Blue value: {}\n", blue);
}
