use std::fs;

struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}

impl Submarine {
    fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }
}

fn main() {
    let mut sub_one = Submarine::new();
    let mut sub_two = Submarine::new();

    // from text file to iter over lines with variable binds for command and value in each line.
    let input = fs::read_to_string("input").expect("Couldn't read file");
    for lines in input.trim().lines() {
        let mut words = lines.split(' ');
        let cmd = words.next().unwrap();
        let val: i32 = words.next().unwrap().parse().unwrap();

        // Part one
        match (cmd, val) {
            ("forward", val) => sub_one.x += val,
            ("up", val) => sub_one.y -= val,
            ("down", val) => sub_one.y += val,
            _ => panic!("Not a valid command!"),
        }

        // Part two
        match (cmd, val) {
            ("forward", val) => {
                sub_two.x += val;
                sub_two.y += val * sub_two.aim;
            }
            ("up", val) => sub_two.aim -= val,
            ("down", val) => sub_two.aim += val,
            _ => panic!("Not a valid command!"),
        }
    }

    println!("{}", sub_one.x * sub_one.y);
    println!("{}", sub_two.x * sub_two.y);
}
