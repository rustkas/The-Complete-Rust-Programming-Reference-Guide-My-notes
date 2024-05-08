trait Driver {
    fn drive(&self) {
        println!("Driver's driving!");
    }
}

struct MyCar;

impl MyCar {
    pub fn drive(&self) {
        println!("I'm driving!");
    }
}

impl Driver for MyCar {}

fn main() {
    {
        let car: &dyn Driver = &MyCar;
        car.drive();
    }
    {
        let car = &MyCar;
        car.drive();
    }
}
