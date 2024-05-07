use std::ops::Add;

pub fn add_thing<T: Add>(fst: T, snd: T) {
    let _ = fst + snd;
}

#[cfg(test)]
mod tests {
    use super::add_thing;

    #[test]
    fn test_add_thing_with_integers() {
        add_thing(5, 3); // We don't need to assert anything as the function only performs addition
    }

    #[test]
    fn test_add_thing_with_floats() {
        add_thing(3.14, 1.59); // We don't need to assert anything
    }


}
