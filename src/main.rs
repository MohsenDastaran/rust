mod inventory;
use fake::{Fake, Faker};

use crate::inventory::products::ProductCategory;

use std::collections::*;

use std::{
    fmt,
    io::{stdin, stdout},
};
fn main() {
    let fake_items: inventory::Item = Faker.fake();
    println!("{:?}", fake_items);

    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);
}
