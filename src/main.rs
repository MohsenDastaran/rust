


#[derive(Debug)]
enum  PaymentMethodType {
   CreditType(String),
   Paypal{username: String, password: String} // struct variant

}

fn main() {
let visa: PaymentMethodType = PaymentMethodType::CreditType(String::from("4544-6545-654-654"));
    println!("{:?}", visa);
    let pay: PaymentMethodType = PaymentMethodType::Paypal { username: String::from("654"), password: String::from("4544-654dfsdvsdvsdvsdv5-654-654") };
    println!("{:?}",pay);
}
