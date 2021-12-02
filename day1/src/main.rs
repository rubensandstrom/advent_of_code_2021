use std::fs;

fn main() {
    // Read in text from input file and make a vec of i32.
    let input: Vec<i32> = fs::read_to_string("input")
        .expect("Couldn't read file")
        .split('\n')
        .map(|x| x.parse().unwrap_or(0))
        .collect();

    // Part one.
    println!("{}", part1(&input));

    // Part two.
    println!("{}", part2(&input));
}

// Returns the number of times input[i+1] is bigger than input[i] in a vec.
fn part1(input: &Vec<i32>) -> i32 {
    let mut counter = 0;
    for i in input.windows(2) {
        if i[0] < i[1] { counter += 1; }
    }
    counter
}

// returns number of times input[i+3] is bigger than input[i] in a vec, which is the same as
// when input[i+1] + input[i+2] + input[i+3] is bigger than input[i] + input[i+1] + input[i+2].
fn part2(input: &Vec<i32>) -> i32 {
    let mut counter = 0;
    for i in input.windows(4) {
        if i[0] < i[3] { counter += 1; }
    }
    counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(&vec!(1, 2, 3, 4, 5, 6)), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&vec!(1, 2, 3, 4, 5, 6)), 3);
    }
}
