mod inventory;

use inventory::products as P; // for removing the inventory:: prefix for manager
fn main() {
    // println!("Hey {MANAGER}") // cannot access manager
    // println!("Hey {}", inventory::MANAGER);
    // println!("Hey {}", inventory::products::FLOOR_SPACE);

    // inventory::talk_to_manager();
    let a = P::Item::new("name".into(), P::ProductCategory::Hammer, 12);
}
