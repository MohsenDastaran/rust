mod inventory;

fn main() {
    // println!("Hey {MANAGER}") // cannot access manager
    println!("Hey {}", inventory::MANAGER);
    println!("Hey {}", inventory::products::FLOOR_SPACE);
    inventory::talk_to_manager();
}
