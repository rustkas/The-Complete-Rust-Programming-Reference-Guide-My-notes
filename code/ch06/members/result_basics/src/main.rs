use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("data.txt");
    let file_result = File::open(&path);
    match file_result {
        Ok(mut file) => {
            let mut s = String::new();
            if let Err(err) = file.read_to_string(&mut s) {
                println!("Error reading file: {}", err);
            } else {
                println!("Message: {}", s);
            }
        }
        Err(err) => {
            println!("Error opening file: {}", err);
        }
    }
}
