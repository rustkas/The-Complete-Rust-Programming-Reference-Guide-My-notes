use std::env;


fn main() {
    if let Ok(rust_compiler_type) = env::var("RUSTC_CHANNEL") {
        println!("Rust compiler type = {rust_compiler_type}");
    }else {
        let result = env::var("RUSTC_CHANNEL");
        println!("{result:?}");
    }
    
    
}