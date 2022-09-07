use crate::second::List;

mod first;
mod second;
mod third;
mod fourth;
mod double;

fn main() {
    let mut list = List::new();
    list.push("olivier");
    list.push("abrivard");
    let elem = list.pop().unwrap_or("error");
    println!("Hello, world {}! ", elem);
}
