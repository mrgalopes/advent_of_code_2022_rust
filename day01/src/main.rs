use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/input1.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut elves: Vec<Vec<u32>> = vec![];
    let mut elf_calories: Vec<u32> = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed_line = line.trim();

            if trimmed_line == "" {
                elves.push(elf_calories);
                elf_calories = vec![];
            } else {
                let calory: u32 = line.parse().expect("Error parsing number");
                elf_calories.push(calory);
            }
        }
    }

    let mut calories: Vec<u32> = elves
        .iter()
        .map(|elf_cals| elf_cals.iter().sum::<u32>())
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
