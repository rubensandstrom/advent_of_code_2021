use std::fs;

fn cascade (vec: &mut Vec<Vec<(u8, bool)>>, i: usize, j: usize) {
    if vec[i][j].0 >= 10 && vec[i][j].1 == false {

        vec[i][j].1 = true;
        if i > 0 {
            vec[i-1][j].0 += 1;
            cascade(vec, i-1, j);
        }
        if j > 0 {
            vec[i][j-1].0 += 1;
            cascade(vec, i, j-1);
        }
        if i < vec.len() - 1 {
            vec[i+1][j].0 += 1;
            cascade(vec, i+1, j);
        }
        if j < vec[0].len() - 1 {
            vec[i][j+1].0 += 1;
            cascade(vec, i, j+1);
        }
        if i > 0 && j > 0 {
            vec[i-1][j-1].0 += 1;
            cascade(vec, i-1, j-1);
        }
        if i > 0 && j < vec[0].len() - 1 {
            vec[i-1][j+1].0 += 1;
            cascade(vec, i-1, j+1);
        }
        if i < vec.len() - 1 && j > 0 {
            vec[i+1][j-1].0 += 1;
            cascade(vec, i+1, j-1);
        }
        if i < vec.len() - 1 && j < vec[0].len() - 1 {
            vec[i+1][j+1].0 += 1;
            cascade(vec, i+1, j+1);
        }
    }
}

fn tick_tok (vec: &mut Vec<Vec<(u8, bool)>>) -> i32 {
    let mut blink = 0;
    for i in 0..vec.len() {
        for j in 0..vec[0].len() {
            vec[i][j].0 += 1;
        }
    }
    for i in 0..vec.len() {
        for j in 0..vec[0].len() {
            cascade(vec, i, j);
        }
    }
    for i in 0..vec.len() {
        for j in 0..vec[0].len() {
            if vec[i][j].1 == true {
                blink += 1;
                vec[i][j].0 = 0;
                vec[i][j].1 = false;
            }
        }
    }
    return blink;
}

fn same (vec: &Vec<Vec<(u8, bool)>>) -> bool {
    for i in 1..vec.len() {
        for j in 0..vec[0].len() {
            if vec[i][j] != vec[i-1][j] {
                return false;
            }
        }
    }
    for j in 1..vec[0].len() {
        if vec[0][j] != vec[0][j-1] {
            return false;
        }
    }
    true
}


fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");


    //task 1
    let mut vec: Vec<Vec<(u8, bool)>> = vec!();

    for i in input.trim().lines() {
        let mut vec1: Vec<(u8, bool)> = vec!();
        for j in i.chars() {
            vec1.push((j as u8 - 48, false)) // fix later
        }
        vec.push(vec1);
    }

    println!("{:?}", vec);

    let mut val = 0;
    for _i in 0..100 {
        val += tick_tok(&mut vec);
    }

    println!("{:?}", vec);
    println!("{:?}", val);

    // task 2
    let mut vec = vec!();
    for i in input.trim().lines() {
        let mut vec1: Vec<(u8, bool)> = vec!();
        for j in i.chars() {
            vec1.push((j as u8 - 48, false)) // fix later
        }
        vec.push(vec1);
    }

    println!("{:?}", vec);

    let mut cycles = 0;
    while !same(&vec) {
        cycles += 1;
        tick_tok(&mut vec);
    }
    println!("{}", cycles);


}
