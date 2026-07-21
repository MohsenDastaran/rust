use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}
#[derive(Debug, Dummy)]
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
