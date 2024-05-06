pub enum PaymentMode {
    Debit,
    Credit,
    Paypal,
}

fn pay_by_credit(amt: u64) {
    println!("Processing credit payment of {amt}");
}
fn pay_by_debit(amt: u64) {
    println!("Processing debit payment of {amt}");
}
fn paypal_redirect(amt: u64) {
    println!("Redirecting to paypal for amount: {amt}");
}

impl PaymentMode {
    pub fn pay(&self, amount: u64) {
        use PaymentMode::*;
        match self {
            Debit => pay_by_debit(amount),
            Credit => pay_by_credit(amount),
            Paypal => paypal_redirect(amount),
        }
    }
}

pub fn get_saved_payment_mode() -> PaymentMode {
    PaymentMode::Debit
}
