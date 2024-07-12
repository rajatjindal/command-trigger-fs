use std::fs;

#[allow(warnings)]
mod bindings;

fn main() {
    spin_executor::run(async move {
        let contents = fs::read_to_string("src/main.rs");
        println!("file content is {:?}", contents);
    });
}
