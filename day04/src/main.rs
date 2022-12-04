use std::{error::Error, fs::File, io::{BufReader, BufRead}, num::ParseIntError};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Assignment {
    start: u32,
    end: u32
}

fn part_1(input_file: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let pairs_fully_containing = reader.lines()
        .filter_map(|line| {
            if let Ok(line) = line {
                if let Ok(assignments) = parse_line(&line) {
                    Some(assignments)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter(|(a1, a2)| fully_contains(&a1, &a2))
        .count();

    println!("{:?}", pairs_fully_containing);
    
    Ok(())
}

fn part_2(input_file: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let pairs_partially_containing = reader.lines()
        .filter_map(|line| {
            if let Ok(line) = line {
                if let Ok(assignments) = parse_line(&line) {
                    Some(assignments)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter(|(a1, a2)| partially_contains(&a1, &a2))
        .count();

    println!("{:?}", pairs_partially_containing);
    
    Ok(())
}

fn parse_line(line: &str) -> Result<(Assignment, Assignment), Box<dyn Error>> {
    let assignment_strs: Vec<&str> = line.split(",").take(2).collect();
    if assignment_strs.len() < 2 {
        return Err(Box::from("Invalid line: {line}"));
    }

    let mut assignments: [Assignment; 2] = [Assignment{start: 0, end: 0}; 2]; 

    for (i, assignment_str) in assignment_strs.iter().enumerate() {
        let assignment_nums: Result<Vec<u32>, ParseIntError> = assignment_str
            .split("-")
            .take(2)
            .map(|s| s.parse())
            .collect();
        let assignment_nums = assignment_nums?;
        let assignment = Assignment {
            start: assignment_nums[0],
            end: assignment_nums[1]
        };
        assignments[i] = assignment;
    }

    Ok((assignments[0], assignments[1]))
}

fn fully_contains(first: &Assignment, second: &Assignment) -> bool {
    (first.start >= second.start && first.end <= second.end)
    || (second.start >= first.start && second.end <= first.end)
}

fn partially_contains(first: &Assignment, second: &Assignment) -> bool {
    (second.start >= first.start && second.start <= first.end)
    || (second.end >= first.start && second.end <= first.end)
    || fully_contains(first, second)
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
    use crate::{Assignment, fully_contains, parse_line, partially_contains};

    #[test]
    fn should_parse_line() {
        let line = "2-4,6-8";
        let expected = (Assignment { start: 2, end: 4 }, Assignment { start: 6, end: 8 });

        let result = parse_line(line);
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn should_check_if_assignment_is_within_another() {
        let within = (Assignment { start: 2, end: 8 }, Assignment { start: 3, end: 7});

        assert!(fully_contains(&within.0, &within.1));
        assert!(fully_contains(&within.1, &within.0));
    }

    #[test]
    fn should_check_if_assignment_is_partially_within_another() {
        let withins = vec![
            (Assignment { start: 5, end: 7 }, Assignment { start: 7, end: 9}),
            (Assignment { start: 2, end: 8 }, Assignment { start: 3, end: 7}),
            (Assignment { start: 6, end: 6 }, Assignment { start: 4, end: 6}),
            (Assignment { start: 2, end: 6 }, Assignment { start: 4, end: 8})
        ];

        for within in withins.into_iter() {
            println!("{:?}", within);
            assert!(partially_contains(&within.0, &within.1));
            assert!(partially_contains(&within.1, &within.0));
        }
    }
}
