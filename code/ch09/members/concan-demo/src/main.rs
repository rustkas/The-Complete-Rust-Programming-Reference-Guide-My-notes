fn main() {
    println!(concat!("Hello, ", "world","!"));

    let s = concat!("test", 10, 'b', true);
assert_eq!(s, "test10btrue");
}
