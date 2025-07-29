mod icon;

#[skyline::main(name = "icons")]
pub fn main() {
    println!("[DEBUG] Hello, Wild World!");
    icon::init();
}