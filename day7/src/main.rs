use std::fs;

fn main() {

    let mut count: i32 = 0;
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let input = input.split(',');
    for i in input {
        count += i.trim().parse::<i32>().unwrap();
    }
    println!("{}", count);
}
