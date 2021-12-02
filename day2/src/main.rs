use std::fs;


struct Sub {
    horizontal_pos: i32,
    depth: i32,
    aim: i32
}

impl Sub {
    fn new() -> Self {
        Self {
            horizontal_pos: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn part1 (&mut self, input: (&str, i32)) {
        match input {
            ("forward", val) => {
                self.horizontal_pos += val;
            }
            ("up", val) => {
                self.depth -= val;
            }
            ("down", val) => {
                self.depth += val;
            }
            _ => panic!("panic in part1")
        }
    }

    fn part2 (&mut self, input: (&str, i32)) {
        match input {
            ("forward", val) => {
                self.horizontal_pos += val;
                self.depth += self.aim * val;
            }
            ("up", val) => {
                self.aim -= val;
            }
            ("down", val) => {
                self.aim += val;
            }
            _ => panic!("panic in part1")
        }
    }
}

fn format_input(input: &str) -> Vec<(&str, i32)> {

    let mut format_input: Vec<(&str, i32)> = vec!();
    for lines in input.trim().lines() {
        let mut parts = lines.split(' ');
        let cmd = parts
            .next()
            .expect("No string here");
        let val: i32 = parts
            .next()
            .expect("No value here")
            .parse()
            .expect("Not an integer");
        format_input.push((cmd, val));
    }
    format_input
}



fn main() {

    let input = fs::read_to_string("input")
        .expect("Couldn't read file");

    let format_input = format_input(&input);

    let mut sub1 =  Sub::new();
    for i in &format_input {

        sub1.part1(*i);
    }

    let mut sub2 =  Sub::new();
    for i in &format_input {

        sub2.part2(*i);
    }

    println!("{}", sub1.horizontal_pos * sub1.depth);
    println!("{}", sub2.horizontal_pos * sub2.depth);

}