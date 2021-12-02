use std::fs;

fn main() {
    let mut part_one = 0;
    let mut part_two = 0;

    let input: Vec<i32> = fs::read_to_string("input")
        .expect("Couldn't read file")
        .trim()
        .lines()
        .map(|x| x.parse().unwrap_or(0))
        .collect();

    // Part one
    for i in input.windows(2) {
        if i[0] < i[1] {
            part_one += 1
        }
    }

    // Part two
    for i in input.windows(4) {
        if i[0] < i[3] {
            part_two += 1
        }
    }

    println!("{}", part_one);
    println!("{}", part_two);
}
