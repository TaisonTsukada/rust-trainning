mod module_hello {
    pub fn print_hello() {
        println!("Hello, world!");
    }
}

fn main() {
    module_hello::print_hello();
}
