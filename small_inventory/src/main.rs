use small_inventory::{Inventory, Product};

fn main() {
    let mut inventory = Inventory::new(vec![], None);

    inventory.add(Product::new("milk".to_string(), 1, 12.5));
    inventory.add(Product::new("Chocolate".to_string(), 3, 8.));

    println!("Inventory after adding products: ");
    inventory.list();

    inventory.remove();
    inventory.add(Product::new("bread".to_string(), 2, 10.));

    println!("Inventory after a remove and an add: ");
    inventory.list();

    println!("Changing the milk price:");
    let milk_product = inventory.find("milk".to_string());

    match milk_product {
        Some(product) => {
            product.set_price(15.);
            product.print();
        }
        None => println!("No product by this name."),
    }

    println!("Total value: {}", inventory.value());
}
