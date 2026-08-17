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
    product_name: String,
    quantity: i32,
    unit_price: i32,
}

impl Order {
    fn validate_name(s: &str) {
        if s.is_empty() {
            panic!("Product name cannot be empty!");
        }
        if s.len() > 300 {
            panic!("Product name cannot be longer than 300 bytes!");
        }
    }
    fn validate_price(price: i32) {
        if price <= 0 {
            panic!("Price must be strictly greater than 0!");
        }
    }
    fn validate_quantity(q: i32) {
        if q <= 0 {
            panic!("Quantity must be strictly greater than 0!");
        }
    }

    pub fn set_product_name(&mut self, s: String) {
        Order::validate_name(&s);
        self.product_name = s;
    }

    pub fn set_unit_price(&mut self, p: i32) {
        Order::validate_price(p);
        self.unit_price = p;
    }

    pub fn set_quantity(&mut self, q: i32) {
        Order::validate_quantity(q);
        self.quantity = q;
    }

    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Order {
        Self::validate_name(&product_name);
        Self::validate_price(unit_price);
        Self::validate_quantity(quantity);

        Order {
            product_name,
            unit_price,
            quantity,
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }

    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }

    pub fn total(&self) -> i32 {
        self.quantity * self.unit_price
    }
}
