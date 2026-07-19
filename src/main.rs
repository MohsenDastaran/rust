mod inventory;

fn main() {
    // println!("Hey {MANAGER}") // cannot access manager
    println!("Hey {}", inventory::MANAGER);
    inventory::talk_to_manager();
}
