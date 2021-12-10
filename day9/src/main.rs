use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read from file");
    for i in input.lines() {
        for j in i.split(',') {
            println!("{}", j);
        }
    }


    println!("{:?}", input);
}
