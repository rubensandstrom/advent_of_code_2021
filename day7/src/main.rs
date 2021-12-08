use std::fs;

fn main() {

    let mut count: i32 = 0;
    let mut hi = 0;
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let input = input.split(',');
    for i in input.map(|x| x.trim().parse::<i32>().unwrap()) {
        count += i;
        if i > hi { hi = i; }
    }
    println!("{}", count);
    println!("{}", hi);

    let average = count / hi;
    println!("{}", average);

    let input = fs::read_to_string("input").expect("Couldn't read file");
    let input = input.split(',');

    let mut sum = 0;
    for i in input.map(|x| x.trim().parse::<i32>().unwrap()) {
        if average - i >= 0 { sum += average - i; }
        else { sum -= average - i; }
    }

    println!("{}", sum);
}
