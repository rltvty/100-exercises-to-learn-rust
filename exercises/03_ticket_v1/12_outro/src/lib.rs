// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    name: String,
    quantity: u32,
    price: u32
}

fn validate_name(name: &String) {
    if name.is_empty() {
        panic!("Name cannot be empty")
    }
    if name.len() > 300 {
        panic!("Name cannot be longer than 300 bytes")
    }
}

fn validate_quantity(quantity: &u32) {
    if *quantity == 0 {
        panic!("Quantity must be greater than zero")
    }
}

fn validate_price(price: &u32) {
    if *price == 0 {
        panic!("Price must be greater than zero")
    }
}

impl Order {
    pub fn new(name: String, quantity: u32, price: u32) -> Order {
        validate_name(&name);
        validate_quantity(&quantity);
        validate_price(&price);
        Order{
            name,
            quantity,
            price
        }
    }

    pub fn product_name(&self) -> &String {
        &self.name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.price
    }

    pub fn set_product_name(&mut self, new_name: String) {
        validate_name(&new_name);
        self.name = new_name
    }

    pub fn set_quantity(&mut self, new_quantity: u32) {
        validate_quantity(&new_quantity);
        self.quantity = new_quantity
    }

    pub fn set_unit_price(&mut self, new_price: u32) {
        validate_price(&new_price);
        self.price = new_price
    }

    pub fn total(&self) -> u32 {
        self.price * self.quantity
    }
}