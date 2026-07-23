use std::{collections::HashMap, fmt::format};

fn main() {
    let mut hotel = Hotel::new("California");
    book_for_one_night(&mut hotel, "Guest 1");
    println!("{:?}", hotel);

    let mut air_bnb = AirBnB::new("Mohsen");
    book_for_one_night(&mut air_bnb, "Guest 1");
    println!("{:?}", air_bnb);
    mix_and_match(&mut hotel, &mut air_bnb, "Gholam");
    println!("{:?} {:?}", hotel, air_bnb);
}

// T is Accomomdation trait here
fn book_for_one_night<T: Accomomdation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}
fn mix_and_match(
    first: &mut (impl Accomomdation + Description),
    second: &mut impl Accomomdation,
    guest: &str,
) {
    first.book(guest, 1);
    second.book(guest, 1);
}

trait Accomomdation {
    fn book(&mut self, name: &str, nights: u32);
}
trait Description {
    fn get_description(&self) -> String;
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
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}
impl Description for Hotel {
    fn get_description(&self) -> String {
        format!("Welcome to the Hotel {}", self.name)
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
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights))
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please Enjoy {}'s Room", self.host)
    }
}
