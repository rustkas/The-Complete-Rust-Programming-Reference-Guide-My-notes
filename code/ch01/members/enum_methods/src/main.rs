use enum_methods::get_saved_payment_mode;

fn main() {
    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(512);
  }
  