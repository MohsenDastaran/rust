mod inventory;

use inventory::MANAGER; // for removing the inventory:: prefix for manager
fn main() {
    // println!("Hey {MANAGER}") // cannot access manager
    println!("Hey {}", MANAGER);
    println!("Hey {}", inventory::products::FLOOR_SPACE);
    inventory::talk_to_manager();
}
