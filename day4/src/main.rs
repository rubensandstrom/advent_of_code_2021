use std::fs;

fn main() {

    let input = fs::read_to_string("input").expect("Couldn't find file");
    let mut input = input.trim().split("\n\n");

    let draw: Vec<i32> = input
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut vec: Vec<Vec<(i32, bool)>> = vec![];
    for lines in input {
        let board = lines
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .map(|x| (x, false))
            .collect();
        vec.push(board)
    }

    let mut win_num = 0;
    let mut win_board = vec![];
    for i in 0..20 {
        let (b, winning_num, winning_board) = new_draw(i * 5, draw.clone(), &mut vec.clone());
        if b == false {
            continue;
        } else {
            win_num = winning_num.unwrap();
            win_board = winning_board.unwrap();
            break;
        }
    }

    let mut count = 0;
    for i in 0..win_board.len() {
        if win_board[i].1 == true {
            count += 1;
        }
    }

    println!("{}", count);
    println!("{}", count * win_num);
}

fn new_draw(
    n: usize,
    draw: Vec<i32>,
    boards: &mut Vec<Vec<(i32, bool)>>,
) -> (bool, Option<i32>, Option<Vec<(i32, bool)>>) {
    let mut bingo = false;
    let mut winning_num: Option<i32> = None;
    let mut winning_board: Option<Vec<(i32, bool)>> = None;
    for i in 0..5 {
        for j in 0..boards.len() {
            for k in 0..boards[0].len() {
                if draw[n + i] == boards[j][k].0 {
                    boards[j][k].1 = true;
                    match check_row(k, boards[j].clone()) || check_col(k, boards[j].clone()) {
                        true => {
                            bingo = true;
                            winning_num = Some(boards[j][k].0);
                            winning_board = Some(boards[j].clone());
                        }
                        false => {}
                    }
                }
            }
        }
    }
    return (bingo, winning_num, winning_board);
}

fn check_row(i: usize, board: Vec<(i32, bool)>) -> bool {
    for j in 0..5 {
        if board[(i % 5) + (5 * j)].1 == false {
            return false;
        }
    }
    return true;
}

fn check_col(i: usize, board: Vec<(i32, bool)>) -> bool {
    for j in 0..5 {
        if board[i - (i % 5) + j].1 == false {
            return false;
        }
    }
    return true;
}
