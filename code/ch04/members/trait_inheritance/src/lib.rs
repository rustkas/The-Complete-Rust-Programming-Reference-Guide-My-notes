pub trait Vehicle {
    fn get_price(&self) -> u64;
}

pub trait Car: Vehicle {
    fn get_model(&self) -> String;
}

pub struct TeslaRoadster {
    model: String,
    pub release_date: u16
}

impl TeslaRoadster {
   pub fn new(model: &str, release_date: u16) -> Self {
        Self { model: model.to_string(), release_date }
    }
}
// Implement Vehicle trait for TeslaRoadster
impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        // Replace this with actual logic to determine the price
        // This example just returns a placeholder value
        return 100_000; // Placeholder price
    }
}
impl Car for TeslaRoadster {
    fn get_model(&self) -> String {
        self.model.clone()
    }
}
