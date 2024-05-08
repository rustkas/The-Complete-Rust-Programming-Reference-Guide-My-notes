use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);

    let value = map.get("one");
    if let Some(value) = value {
        let incremented_value = *value + 1;
        println!("Incremented value: {incremented_value}" );
    } else {
        println!("Key 'one' not found in the map");
    }
}
