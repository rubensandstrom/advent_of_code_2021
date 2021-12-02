use std::fs;


// Super messy!

fn main() {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    let input = fs::read_to_string("input")
        .expect("Couldn't read file");

    let input1: Vec<&str> = input
        .trim()
        .lines()
        .collect();

    let mut input2: Vec<Vec<&str>> = vec!();
    for i in input1 {

        input2.push(i.split(' ').collect());
    }
    let mut input3 = vec!();
    for i in input2.iter() {
        let tup: (&str, i32) = (i[0], i[1].parse().expect("not a number"));
        input3.push(tup);
    }

    for i in &input3 {
        change_position(&mut horizontal_pos, &mut depth, &i);
    }

    println!("part one: {}", horizontal_pos*depth);

    let mut aim = 0;
    depth = 0;
    horizontal_pos = 0;

    for i in input3 {
        change_position_with_aim(&mut aim, &mut horizontal_pos, &mut depth, &i);
    }
    println!("part one: {}", horizontal_pos*depth);

}

fn change_position ( horizontal_pos: &mut i32, depth: &mut i32, command: &(&str, i32)) {
    match command {
        ("forward", val) => { *horizontal_pos += val }
        ("down", val) => { *depth += val }
        ("up", val) => { *depth -= val }
        _ => panic!("Unexpected command")
    }
}

fn change_position_with_aim (aim: &mut i32, horizontal_pos: &mut i32, depth: &mut i32, &command: &(&str, i32)) {
    match command {
        ("forward", val) => { *horizontal_pos += val; *depth += val * *aim; }
        ("down", val) => { *aim += val; }
        ("up", val) => { *aim -= val; }
        _ => panic!("Unexpected command")
    }
}