pub struct Product {
    name: String,
    quantity: u32,
    price: f32,
}

impl Product {
    pub fn new(name: String, quantity: u32, price: f32) -> Product {
        Product {
            name,
            quantity,
            price,
        }
    }

    pub fn set_quantity(&mut self, value: u32) {
        self.quantity = value;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    pub fn print(&self) {
        println!(
            "Name: {} | Quantity: {} | Price: {}",
            self.name, self.quantity, self.price
        );
    }
}

pub struct Inventory {
    products: Vec<Product>,
    total_value: f32,
}

impl Inventory {
    pub fn new(products: Vec<Product>, total_value: Option<f32>) -> Inventory {
        match total_value {
            Some(value) => Inventory {
                products,
                total_value: value,
            },
            None => Inventory {
                products,
                total_value: 0.,
            },
        }
    }

    pub fn value(&self) -> f32 {
        self.total_value
    }

    pub fn add(&mut self, product: Product) {
        self.total_value += product.price;
        self.products.push(product);
    }

    pub fn remove(&mut self) -> Option<Product> {
        let result = self.products.pop();
        match result {
            Some(product) => {
                self.total_value -= product.price;
                Some(product)
            }
            None => None,
        }
    }

    pub fn find(&mut self, name: String) -> Option<&mut Product> {
        let result = self.products.iter_mut().find(|p| p.name == name);
        match result {
            Some(product) => Some(product),
            None => None,
        }
    }

    pub fn list(&self) {
        for prod in &self.products {
            prod.print();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Product tests

    #[test]
    fn test_product_creation() {
        let product = Product::new("Laptop".to_string(), 10, 999.99);

        assert_eq!(product.get_name(), "Laptop");
        assert_eq!(product.get_quantity(), 10);
        assert_eq!(product.get_price(), 999.99);
    }

    #[test]
    fn test_product_setters() {
        let mut product = Product::new("Laptop".to_string(), 10, 999.99);

        product.set_name("Desktop".to_string());
        product.set_quantity(5);
        product.set_price(1499.99);

        assert_eq!(product.get_name(), "Desktop");
        assert_eq!(product.get_quantity(), 5);
        assert_eq!(product.get_price(), 1499.99);
    }

    // Inventory tests

    #[test]
    fn test_inventory_creation_with_value() {
        let inventory = Inventory::new(vec![], Some(500.0));

        assert_eq!(inventory.value(), 500.0);
    }

    #[test]
    fn test_inventory_creation_without_value() {
        let inventory = Inventory::new(vec![], None);

        assert_eq!(inventory.value(), 0.0);
    }

    #[test]
    fn test_inventory_add_product() {
        let mut inventory = Inventory::new(vec![], None);

        let product = Product::new("Mouse".to_string(), 10, 25.0);
        inventory.add(product);

        assert_eq!(inventory.value(), 25.0);
    }

    #[test]
    fn test_inventory_remove_product() {
        let mut inventory = Inventory::new(vec![], None);

        inventory.add(Product::new("Keyboard".to_string(), 5, 50.0));

        let removed = inventory.remove();

        assert!(removed.is_some());
        assert_eq!(removed.unwrap().get_name(), "Keyboard");
        assert_eq!(inventory.value(), 0.0);
    }

    #[test]
    fn test_inventory_remove_empty() {
        let mut inventory = Inventory::new(vec![], None);

        let removed = inventory.remove();

        assert!(removed.is_none());
        assert_eq!(inventory.value(), 0.0);
    }

    #[test]
    fn test_inventory_find_product() {
        let mut inventory = Inventory::new(vec![], None);

        inventory.add(Product::new("Monitor".to_string(), 3, 200.0));

        let product = inventory.find("Monitor".to_string());

        assert!(product.is_some());
        assert_eq!(product.unwrap().get_name(), "Monitor");
    }

    #[test]
    fn test_inventory_find_nonexistent_product() {
        let mut inventory = Inventory::new(vec![], None);

        let product = inventory.find("Phone".to_string());

        assert!(product.is_none());
    }

    #[test]
    fn test_inventory_multiple_products() {
        let mut inventory = Inventory::new(vec![], None);

        inventory.add(Product::new("Mouse".to_string(), 10, 25.0));
        inventory.add(Product::new("Keyboard".to_string(), 5, 50.0));
        inventory.add(Product::new("Monitor".to_string(), 3, 200.0));

        assert_eq!(inventory.value(), 275.0);

        inventory.remove();

        assert_eq!(inventory.value(), 75.0);
    }
}
