use std::string::FromUtf8Error;

fn str_upper_concise(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = String::from_utf8(str).map(|s| s.to_uppercase())?;
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

fn main() {
    let valid_str = str_upper_concise(vec![89, 97, 89]);
    println!("{:?}", valid_str);
    if let Ok(string) = valid_str {
        for character in string.chars() {
            println!("{}", character);
        }
    }
}
