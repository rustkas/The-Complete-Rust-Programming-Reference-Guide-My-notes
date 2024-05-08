use c_macros_rust::switch;

fn main() {
    let mut x = 1;
    let mut y = 2;

    switch!(x, y);
    println!("{result:?}", result = (x, y));
}
