use crate::traits::{Executable, Resettable};
use crate::result::CartResult;

pub struct Cart {
    pub name: String,
    pub items: Vec<String>,
}

impl Cart {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: vec![],
        }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(item.to_string());
    }
}

impl Resettable for Cart {
    fn reset(&mut self) {
        self.items.clear();
    }
}

impl Executable for Cart {
    fn execute(&self) -> CartResult<()> {
        println!("Executing cart: {}", self.name);
        Ok(())
    }
}
