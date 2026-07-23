use std::{collections::HashMap, fmt::format};

fn main() {
    let mut hotel = Hotel::new("California");
    book_for_one_night(&mut hotel, "Guest 1");
    println!("{:?}", hotel);

    let mut air_bnb = AirBnB::new("Mohsen");
    book_for_one_night(&mut air_bnb, "Guest 1");
    println!("{:?}", air_bnb);
}
//  Accomomdation here means entity will be any types that implements Accomomdation trait (Hotel & AirBnB)
fn book_for_one_night(entity: &mut impl Accomomdation, guest: &str) {
    println!("{}", entity.get_description());
    entity.book(guest, 1);
}
trait Accomomdation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}
#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}
impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accomomdation for Hotel {
    fn get_description(&self) -> String {
        format!("Welcome to the Hotel {}", self.name)
    }
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}
#[derive(Debug)]

struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}
impl Accomomdation for AirBnB {
    fn get_description(&self) -> String {
        format!("Please Enjoy {}'s Room", self.host)
    }
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights))
    }
}
