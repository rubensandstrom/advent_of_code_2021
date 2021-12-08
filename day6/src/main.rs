use std::fs;

#[derive(Debug)]
pub struct Lanternfish(u64, u64, u64, u64, u64, u64, u64, u64, u64);

impl Lanternfish {
    pub fn new() -> Self {
        Self(0, 0, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn tick_tock(&mut self) {
        let zero = self.0;
        self.0 = self.1;
        self.1 = self.2;
        self.2 = self.3;
        self.3 = self.4;
        self.4 = self.5;
        self.5 = self.6;
        self.6 = self.7 + zero;
        self.7 = self.8;
        self.8 = zero;
    }

    pub fn display(&self) {
        let mut sum = 0;
        sum += self.0;
        sum += self.1;
        sum += self.2;
        sum += self.3;
        sum += self.4;
        sum += self.5;
        sum += self.6;
        sum += self.7;
        sum += self.8;
        println!("{}", sum);
    }
}

fn main() {
    let mut fish = Lanternfish::new();

    let input = fs::read_to_string("input").expect("Couldn't read file");
    for i in input.trim().split(',') {
        match i {
            "8" => fish.8 += 1,
            "7" => fish.7 += 1,
            "6" => fish.6 += 1,
            "5" => fish.5 += 1,
            "4" => fish.4 += 1,
            "3" => fish.3 += 1,
            "2" => fish.2 += 1,
            "1" => fish.1 += 1,
            "0" => fish.0 += 1,
            _ => panic!("non valid input"),
        }
    }

    // Part 1
    for _i in 0..80 {
        fish.tick_tock();
    }
    fish.display();

    // Part 2
    for _i in 80..256 {
        fish.tick_tock()
    }
    fish.display();
}
