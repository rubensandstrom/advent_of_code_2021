use std::fs;

fn err_score (vec: Vec<Option<char>>) -> i32 {
    let mut score  = 0;
    for c in vec {
        match c {
            Some(')') => { score += 3; }
            Some(']') => { score += 57; }
            Some('}') => { score += 1197; }
            Some('>') => { score += 25137; }
            _ => {}
        }
    }
    return score;
}

fn syntax_err (line: &str) -> Option<char> {
    let mut stack: Vec<char> = vec!();
    for c in line.chars() {
        match Some(c) {
            Some('(') => { stack.push(c); }
            Some('[') => { stack.push(c); }
            Some('{') => { stack.push(c); }
            Some('<') => { stack.push(c); }
            Some(')') => {
                let pop = stack.pop();
                if pop == Some('(') { continue; }
                else { return Some(')'); }
            }
            Some(']') => {
                let pop = stack.pop();
                if pop == Some('[') { continue; }
                else { return Some(']'); }
            }
            Some('}') => {
                let pop = stack.pop();
                if pop == Some('{') { continue; }
                else { return Some('}'); }
            }
            Some('>') => {
                let pop = stack.pop();
                if pop == Some('<') { continue; }
                else { return Some('>'); }
            }
            _ => {}
        }

    }
    return None
}

fn auto_complete (line: &str) -> Option<Vec<char>> {
    let mut stack: Vec<char> = vec!();
    for c in line.chars() {
        match Some(c) {
            Some('(') => { stack.push(c); }
            Some('[') => { stack.push(c); }
            Some('{') => { stack.push(c); }
            Some('<') => { stack.push(c); }
            Some(')') => {
                let pop = stack.pop();
                if pop == Some('(') { continue; }
                else { return None; }
            }
            Some(']') => {
                let pop = stack.pop();
                if pop == Some('[') { continue; }
                else { return None; }
            }
            Some('}') => {
                let pop = stack.pop();
                if pop == Some('{') { continue; }
                else { return None; }
            }
            Some('>') => {
                let pop = stack.pop();
                if pop == Some('<') { continue; }
                else { return None; }
            }
            _ => {}
        }

    }
    let mut vec = vec!();

    while stack.len() > 0 {
        let pop = stack.pop();
        match pop {
            Some('(') => { vec.push(')'); }
            Some('[') => { vec.push(']'); }
            Some('{') => { vec.push('}'); }
            Some('<') => { vec.push('>'); }
            _ => {return None}

        }
    }
    Some(vec)
}

fn complete (comp: Option<Vec<char>>) -> Option<i64> {
    let mut score: i64 = 0;
    for c in comp.unwrap_or(vec!('X')) {
        match c {
            ')' => {
                score *= 5;
                score += 1;
            }
            ']' => {
                score *= 5;
                score += 2;
            }
            '}' => {
                score *= 5;
                score += 3;
            }
            '>' => {
                score *= 5;
                score += 4;
            }
            _ => { return None}
        }
    }
    return Some(score);
}

fn main() {


    let input = fs::read_to_string("input").expect("Couldn't read file");

    let mut err_vec: Vec<Option<char>> = vec!();
    let mut complete_score: Vec<i64> = vec!();

    for lines in input.trim().lines() {
        err_vec.push(syntax_err(lines));
    }
    println!("{}", err_score(err_vec));

    let input = fs::read_to_string("input").expect("Couldn't read file");

    for lines in input.trim().lines() {
        let val = complete(auto_complete(lines));
        if val == None {
            continue;
        }
        else {
            complete_score.push(val.unwrap());
        }
    }

    complete_score.sort();
    println!("{}", complete_score[(complete_score.len() -1) / 2])


}

#[cfg(test)]
mod test {
    use crate::syntax_err;
    use super::err_score;

    #[test]
    fn test1 () {
        assert_eq!(err_score(vec!(Some(')'), Some(']'), Some('>'), None, Some('}'))), 3+57+1197+25137);
    }

    #[test]
    fn test2 () {
        assert_eq!(syntax_err("()((<>{()})]()<>{<()>}"), Some(']'));
    }
}