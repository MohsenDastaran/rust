pub const FLOOR_SPACE: i32 = 10;
pub const MANAGER: &str = "Dastaran";
pub enum ProductCategory {
    Ladder,
    Hammer,
}
pub struct Item {
    name: String,
    category: ProductCategory,
    quantity: u32,
}
impl Item {
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        super::talk_to_manager(); // get parent module from child
        Self {
            name,
            category,
            quantity,
        }
    }
}
