mod inventory {
    pub const FLOOR_SPACE: i32 = 10000;
    pub const MANAGER: &str = "Mohsen Dastaran";

    #[derive(Debug)]
    enum ProductCategory {
        Ladder,
        Hammer,
    }
    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }

    pub fn talk_to_manager() {
        println!("Hey {MANAGER}, Hahahaha")
    }
}

fn main() {
    // println!("Hey {MANAGER}") // cannot access manager
    println!("Hey {}", inventory::MANAGER);
    inventory::talk_to_manager();
}
