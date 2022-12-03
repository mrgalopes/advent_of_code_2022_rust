use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn part_1(input_file: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let priority_sum = reader
        .lines()
        .map(|line| {
            if let Ok(line) = line {
                find_duplicate(&line)
            } else {
                None
            }
        })
        .filter_map(|letter| letter)
        .map(|letter| priority(letter))
        .sum::<u32>();

    println!("{priority_sum}");

    Ok(())
}

fn part_2(input_file: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut i = 0;
    let mut group: [String; 3] = ["".into(), "".into(), "".into()];
    let mut common_letters = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        group[i] = line?;
        i += 1;
        if i == 3 {
            if let Some(letter) = find_common_letter(&group[0], &group[1], &group[2]) {
                common_letters.push(letter);
            } else {
                println!("Not found! {:?}", group);
                println!("Line number {}", line_number);
            }
            i = 0;
        }
    }

    let priority_sum = common_letters
        .into_iter()
        .map(|letter| priority(letter))
        .sum::<u32>();

    println!("{priority_sum}");

    Ok(())
}

fn find_common_letter(first_group: &str, second_group: &str, third_group: &str) -> Option<char> {
    for letter in first_group.chars() {
        if second_group.contains(letter) && third_group.contains(letter) {
            return Some(letter);
        }
    }

    None
}

fn find_duplicate(items: &str) -> Option<char> {
    let half_index = items.len() / 2;
    let first_half = &items[..half_index];
    let second_half = &items[half_index..];

    for letter in first_half.chars() {
        if second_half.contains(letter) {
            return Some(letter);
        }
    }

    None
}

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => (item as u32) - ('a' as u32) + 1,
        'A'..='Z' => (item as u32) - ('A' as u32) + 27,
        _ => 0,
    }
}

fn main() {
    if let Err(err) = part_1("input/input.txt") {
        println!("{:?}", err);
    }

    if let Err(err) = part_2("input/input.txt") {
        println!("{:?}", err);
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_duplicate, priority};

    #[test]
    fn should_calculate_priority() {
        assert_eq!(16, priority('p'));
        assert_eq!(38, priority('L'));
        assert_eq!(42, priority('P'));
        assert_eq!(22, priority('v'));
        assert_eq!(20, priority('t'));
        assert_eq!(19, priority('s'));
    }

    #[test]
    fn should_find_duplicate_item() {
        let results = vec![
            ("vJrwpWtwJgWrhcsFMMfFFhFp", 'p'),
            ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L'),
        ];

        for (items, result) in results.into_iter() {
            let duplicate = find_duplicate(items);
            assert!(duplicate.is_some());
            assert_eq!(result, duplicate.unwrap());
        }
    }
}
