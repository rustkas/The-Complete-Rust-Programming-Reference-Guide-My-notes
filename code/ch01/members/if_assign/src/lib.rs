pub fn get_result(a: u8, b: u8) -> &'static str {
    let result = if a == b {
        "Wait, what?"
    } else {
        "Rust makes sense"
    };
    result
}
