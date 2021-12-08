use std::collections::HashMap;
use std::fs;

fn main() {
    // This part just parses input into format Vec<Vec<(i32, i32)>>
    let input = fs::read_to_string("input").expect("Couldn't find file");
    let mut lines: Vec<Vec<(i32, i32)>> = vec![];
    for i in input.trim().lines() {
        let mut vec: Vec<(i32, i32)> = vec![];
        for j in i.split(" -> ") {
            let mut k = j.split(',');
            vec.push((
                k.next().unwrap().parse().unwrap(),
                k.next().unwrap().parse().unwrap(),
            ));
        }
        lines.push(vec);
    }

    let mut count = 0;
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    for i in lines {
        let input = vert_or_horiz(i);
        match input {
            Some(line) => {
                for j in line {
                    if map.get(&j).unwrap_or(&2) == &1 {
                        count += 1;
                        map.insert(j, 2); // We only want to count intersections not how many streams that intersect in a given point.
                    } else {
                        map.insert(j, 1);
                    }
                }
            }
            None => {}
        }
    }

    println!("{:?}", map);
    println!("{}", count); // Wrong answer: 1295 to low
                           // wrong answer 6012 to high
}

fn vert_or_horiz(line: Vec<(i32, i32)>) -> Option<Vec<(i32, i32)>> {
    let mut vec: Vec<(i32, i32)> = vec![];
    if line[0].0 == line[1].0 {
        if line[0].1 < line[1].1 {
            for i in line[0].1..=line[1].1 {
                vec.push((line[0].0, i));
            }
        } else {
            for i in line[1].1..=line[0].1 {
                vec.push((line[0].0, i));
            }
        }
    } else if line[0].1 == line[1].1 {
        if line[0].0 < line[1].0 {
            for i in line[0].0..=line[1].0 {
                vec.push((i, line[0].1));
            }
        } else {
            for i in line[1].0..=line[0].0 {
                vec.push((i, line[0].1));
            }
        }
    } else if line[0].0 == line[1].0 && line[0].1 == line[1].1 {
        return Some(line);
    } else {
        return None;
    }
    return Some(vec);
}

#[cfg(test)]
mod test {
    use super::vert_or_horiz;

    #[test]
    fn test1() {
        assert_eq!(
            vert_or_horiz(vec!((5, 3), (5, 6))),
            Some(vec!((5, 3), (5, 4), (5, 5), (5, 6)))
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            vert_or_horiz(vec!((5, 6), (5, 3))),
            Some(vec!((5, 3), (5, 4), (5, 5), (5, 6)))
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            vert_or_horiz(vec!((1, 9), (5, 9))),
            Some(vec!((1, 9), (2, 9), (3, 9), (4, 9), (5, 9)))
        );
    }
    #[test]
    fn test4() {
        let mut vec = vec![];
        for i in 1..=1001 {
            vec.push((i, 9))
        }
        assert_eq!(vert_or_horiz(vec!((1001, 9), (1, 9))), Some(vec));
    }
    #[test]
    fn test5() {
        assert_eq!(vert_or_horiz(vec!((5, 10), (100, 91))), None);
    }
    #[test]
    fn test6() {
        assert_eq!(
            vert_or_horiz(vec!((1, 2), (1, 3))),
            Some(vec!((1, 2), (1, 3)))
        );
    }
    #[test]
    fn test7() {
        assert_eq!(vert_or_horiz(vec!((1, 2), (1, 2))), Some(vec!((1, 2))));
    }
}
