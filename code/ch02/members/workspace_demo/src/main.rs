mod food {
    #[derive(Debug)]
    pub struct Cake;
    
    #[allow(dead_code)]
    struct Smoothie;
    
    #[allow(dead_code)]
    struct Pizza;
}

use food::Cake;

fn main() {
    let eatable = Cake;
    println!("{eatable:?}");
}
