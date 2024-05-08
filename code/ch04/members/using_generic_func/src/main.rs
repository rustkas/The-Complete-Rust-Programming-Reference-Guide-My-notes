fn main() {
    {
        let num_from_str = str::parse::<u8>("34").unwrap();
        println!("Parsed number {num_from_str}");
    }

    {
        // providing a type
        let v1: Vec<u8> = Vec::new();
        drop(v1);
        // or calling method
        let mut v2 = Vec::new();
        v2.push(2); // v2 is now Vec<i32>

        // or using turbofish
        let v3 = Vec::<u8>::new(); // not so readable
        drop(v3);
    }
}
