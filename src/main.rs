#[derive(Debug)]
enum  PaymentMethodType {
   CreditType(String),
   Paypal(String)

}

fn main() {
let visa = PaymentMethodType::CreditType(String::from("4544-6545-654-654"));
    println!("{:?}", visa);

}
