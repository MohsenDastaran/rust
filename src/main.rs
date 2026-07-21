mod inventory;
use fake::{Fake, Faker};
fn main() {
    let fake_items: inventory::Item = Faker.fake();
    println!("{:?}", fake_items);
}
