// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
pub struct Order {
    product_name : String,
    quantity : u32,
    unit_price : u32,
}
impl Order {
    pub fn new(product_name : String, quantity : u32, unit_price : u32) -> Order {
        Order {
            product_name : Self::valid_product_name(product_name),
            quantity : Self::valid_quantity(quantity),
            unit_price : Self::valid_unit_price(unit_price),
        }
    }

    //   The product name can't be empty and it can't be longer than 300 bytes.
    fn valid_product_name(a_product_name : String) -> String{
        if a_product_name.is_empty() {
            panic!("Title cannot be empty");
        }
        if a_product_name.len() > 300 {
            panic!("Title cannot be longer than 50 bytes");
        }
        a_product_name
    }
    //   The quantity must be strictly greater than zero.
    fn valid_quantity(a_quantity : u32) -> u32 {
        if a_quantity == 0 {
            panic!("Title cannot be empty");
        }
        a_quantity
    }
    //   The unit price is in cents and must be strictly greater than zero.
    fn valid_unit_price(a_unit_price : u32) -> u32 {
        if a_unit_price == 0 {
            panic!("Title cannot be empty");
        }
        a_unit_price
    }

    //   Order must include a method named `total` that returns the total price of the order.
    pub fn total(&self) -> u32{
        self.quantity * self.unit_price
    }

    //   Order must provide setters and getters for each field.
    pub fn quantity(&self) -> &u32{
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32{
        &self.unit_price
    }

    pub fn product_name(&self) -> &str{
        &self.product_name
    }

    pub fn set_product_name(&mut self, product_name : String) {
        self.product_name = Self::valid_product_name(product_name);
    }
    pub fn set_quantity(&mut self, quantity : u32) {
        self.quantity = Self::valid_quantity(quantity);
    }

    pub fn set_unit_price(&mut self, unit_price : u32) {
        self.unit_price = Self::valid_unit_price(unit_price);
    }
}




//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
