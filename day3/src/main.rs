use std::fs;

// This function works and passes tests, can probobly be done in a nicer way but that can wait.
fn nth_most(n: usize, vec: &Vec<&str>) -> char {
    let mut val = 0;

    for s in vec {
        if s.chars().nth(n).unwrap() == '1' {
            val += 1
        }
    }
    if val > vec.len() / 2 {
        return '1';
    } else {
        return '0';
    }
}

fn oxygen(n: usize, vec: Vec<&str>) -> &str {
    if vec.len() == 1 {
        return vec.get(0).unwrap();
    }
    if vec.len() == 2 {
        for i in 0..vec.get(0).unwrap().len() {
            if vec.get(0).unwrap().chars().nth(i) == vec.get(1).unwrap().chars().nth(i) {
                continue;
            } else if vec.get(0).unwrap().chars().nth(i).unwrap() == '1' {
                return vec.get(0).unwrap();
            } else {
                return vec.get(1).unwrap();
            }
        }
    }

    let mut new_vec = vec![];
    for i in &vec {
        if i.chars().nth(n).unwrap() == nth_most(n, &vec) {
            new_vec.push(*i)
        }
    }
    return oxygen(n + 1, new_vec);
}

fn co2_scrub(n: usize, vec: Vec<&str>) -> &str {
    if vec.len() == 1 {
        return vec.get(0).unwrap();
    }
    if vec.len() == 2 {
        for i in 0..vec.get(0).unwrap().len() {
            if vec.get(0).unwrap().chars().nth(i) == vec.get(1).unwrap().chars().nth(i) {
                continue;
            } else if vec.get(0).unwrap().chars().nth(i).unwrap() == '0' {
                return vec.get(0).unwrap();
            } else {
                return vec.get(1).unwrap();
            }
        }
    }

    let mut new_vec = vec![];
    for i in &vec {
        if i.chars().nth(n).unwrap() != (nth_most(n, &vec)) {
            new_vec.push(*i)
        }
    }
    return co2_scrub(n + 1, new_vec);
}

fn string_to_int(s: &str) -> i32 {
    let mut val = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '1' {
            val += 1 << s.len() - (i + 1)
        }
    }
    val
}

fn flip(s: &str) -> String {
    let mut new_string = String::from("");
    for c in s.chars() {
        if c == '0' {
            new_string.push('1')
        } else {
            new_string.push('0')
        }
    }
    new_string
}

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't find file");

    let input = input.trim().lines().collect();

    let mut string = String::from("");
    for i in 0..12 {
        string.push(nth_most(i, &input))
    }

    println!("{}", string_to_int(&string) * string_to_int(&flip(&string)));
    println!("{}", string_to_int(oxygen(0, input.clone()))); // 2727
    println!("{}", string_to_int(co2_scrub(0, input.clone()))); // 1601
}

#[cfg(test)]
mod tests {
    use super::nth_most;
    use super::oxygen;
    use super::string_to_int;
    use crate::{co2_scrub, flip};

    #[test]
    fn test1() {
        assert_eq!('1', nth_most(0, &vec!("1010", "0110", "1111")));
    }
    #[test]
    fn test2() {
        assert_ne!('1', nth_most(0, &vec!("1001", "0011", "0101")));
    }

    #[test]
    fn test3() {
        assert_eq!(oxygen(0, vec!("1001", "1111", "1010", "0101")), "1010")
    }
    #[test]
    fn test4() {
        assert_eq!(string_to_int("1001"), 9);
    }
    #[test]
    fn test5() {
        assert_eq!(flip("1001"), "0110");
    }
    #[test]
    fn test6() {
        assert_eq!(string_to_int(&flip("1001")), 6);
    }
    #[test]
    fn test7() {
        assert_eq!(
            co2_scrub(
                0,
                vec!(
                    "1001", "1111", "1010", "0101", "0010", "1101", "1011", "1110", "0001", "0110"
                )
            ),
            "0101"
        )
    }
}
