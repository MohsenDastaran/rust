#[derive(Debug)]
enum  PaymentMethodType {
   CreditType(String),
   Paypal(String, String)

}

fn main() {
let visa: PaymentMethodType = PaymentMethodType::CreditType(String::from("4544-6545-654-654"));
    println!("{:?}", visa);
    let pay: PaymentMethodType = PaymentMethodType::Paypal(String::from("654"), String::from("4544-654dfsdvsdvsdvsdv5-654-654"));
    println!("{:?}",pay);
}
