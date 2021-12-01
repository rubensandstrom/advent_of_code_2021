use std::fs;
fn main() {

    // Read in text from input file and make a vec of i32.
    let input: Vec<i32> = fs::read_to_string("input")
        .expect("Couldn't read file")
        .split('\n')
        .map(|x| x.parse().unwrap_or(0))
        .collect();

    // Part one.
    println!("{}", inc(&input));

    // Part two.
    println!("{}", inc(&sum_of_three(&input)));
}

// Returns the number of times i is bigger than i-1 in a vec.
fn inc( input: &Vec<i32>) -> i32 {
    let mut rv = 0;
    let mut current = input[0];
    for i in input {
        match i - current {
            x if x > 0 => { rv += 1; current = *i;}
            _ => { current = *i;}
        }

    }
    rv
}

// Returns a vec of "three-measurement sliding window".
fn sum_of_three(input: &Vec <i32>) -> Vec<i32> {

    let mut vec = vec!();
    let mut sum = 0;
    for i in 0..=(input.len() - 3) {
        for j in 0..3 {
            sum += input[i + j];
        }
        vec.push(sum);
        sum = 0;
    }
    vec

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(sum_of_three(&vec!(1, 2, 3, 4, 5, 6)), vec!(6, 9, 12, 15));
    }
}