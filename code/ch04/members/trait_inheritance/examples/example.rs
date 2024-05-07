use trait_inheritance::{Car, TeslaRoadster, Vehicle};

fn main() {
  let my_roadster = TeslaRoadster::new("Tesla Roadster II", 2020);
  println!("{} is priced at ${}", my_roadster.get_model(), my_roadster.get_price());
}