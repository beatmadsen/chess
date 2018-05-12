mod direction;
mod error;
mod name;

use name::Name;

fn main() {
    let i: Name = "b2".parse().unwrap();
    println!("Result: {}", i);
}
