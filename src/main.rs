mod direction;
mod error;
mod name;
mod square;

use name::Name;

fn main() {
    let i: Name = "b2".parse().unwrap();
    println!("Result: {}", i);
}
