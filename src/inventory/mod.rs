pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Mohsen Dastaran";
use fake::Dummy;

#[derive(Debug, Dummy)]
enum ProductCategory {
    Ladder,
    Hammer,
}
#[derive(Debug, Dummy)]
pub struct Item {
    name: String,
    category: ProductCategory,
    quantity: u32,
}

pub fn talk_to_manager() {
    println!("Hey {MANAGER}, Hahahaha");
}

pub mod products;
