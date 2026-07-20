mod inventory;

use inventory::products; // for removing the inventory:: prefix for manager
fn main() {
    // println!("Hey {MANAGER}") // cannot access manager
    // println!("Hey {}", inventory::MANAGER);
    // println!("Hey {}", inventory::products::FLOOR_SPACE);

    // inventory::talk_to_manager();
    let a = products::Item::new("name".into(), products::ProductCategory::Hammer, 12);
}
