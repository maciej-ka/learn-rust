use std::io;

fn main() {
    let array = ["foo", "bar", "baz"];

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read");

    let index: usize = index
        .trim()
        .parse()
        .expect("Please type a number");

    let element = array[index];
    println!("value of index {index} is {element}");
}
