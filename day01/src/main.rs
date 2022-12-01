use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn new() -> Elf {
        Elf {
            calories: Vec::new(),
        }
    }
}

fn main() {
    let file = File::open("inputs/input1.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut elves: Vec<Elf> = vec![];

    let mut elf = Elf::new();

    for line in reader.lines() {
        let line = line.expect("Error in line");
        let trimmed_line = line.trim();
        if trimmed_line == "" {
            elves.push(elf.clone());
            elf = Elf::new();
        } else {
            let calory: u32 = line.parse().expect("Error parsing number");
            elf.calories.push(calory);
        }
    }

    let mut calories: Vec<u32> = elves
        .iter()
        .map(|elf| elf.calories.iter().sum::<u32>())
        .collect();
    calories.sort();

    let mut total_calories = 0u32;
    for _ in 0..3 {
        let cal = calories.pop().unwrap_or(0);
        total_calories += cal;
        println!("{:?}", cal);
    }
    println!("{}", total_calories);
}
