use std::fs;

fn main() {
    struct Position {
        x: i32,
        y: i32,
        aim: i32,
    }

    let mut pos1 = Position { x: 0, y: 0, aim: 0 };
    let mut pos2 = Position { x: 0, y: 0, aim: 0 };

    let input = fs::read_to_string("input").expect("Couldn't read file");
    for lines in input.trim().lines() {
        let mut parts = lines.split(' ');
        let cmd = parts.next().unwrap();
        let val: i32 = parts.next().unwrap().parse().unwrap();

        // Part 1
        match (cmd, val) {
            ("forward", val) => pos1.x += val,
            ("up", val) => pos1.y -= val,
            ("down", val) => pos1.y += val,
            _ => panic!(""),
        }

        // Part 2
        match (cmd, val) {
            ("forward", val) => {
                pos2.x += val;
                pos2.y += val * pos2.aim;
            }
            ("up", val) => {
                pos2.aim -= val;
            }
            ("down", val) => pos2.aim += val,
            _ => panic!(""),
        }
    }

    println!("{}", pos1.x * pos1.y);
    println!("{}", pos2.x * pos2.y);
}
