
/// A convenient macro to read input as string into a buffer
#[macro_export]
macro_rules! scanline {
    ($x:expr) => {{
      std::io::stdin().read_line(&mut $x).unwrap();
        let _= $x.trim();
    }};
    () => {{
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s
    }};
}
