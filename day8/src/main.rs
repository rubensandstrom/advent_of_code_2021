use std::fs;

fn main() {

    let input = fs::read_to_string("input").expect("Couldn't read from file");
    let input = input.trim().split(" | ");

    for i in input {

        println!("{}", i);
    }
}
