use if_assign::get_result;

fn main() {
    let array = [(1, 2), (1, 1), (2, 1)];
    for (a, b) in array {
        let result = get_result(a, b);

        println!("You know what? {result}");
    }
}
