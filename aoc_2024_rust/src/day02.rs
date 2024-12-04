use std::fs;
use std::str::FromStr;


pub fn print() {
    let s = fs::read_to_string("data/02.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 2");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Input {
    reports: Vec<Vec<i32>>,
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reports = input.lines().map(|line| {
            line.split(" ").map(|v| i32::from_str(v).expect("cannot parse number in input")).collect::<Vec<_>>()
        }).collect::<Vec<Vec<i32>>>();
        Ok(Input {
            reports
        })
    }
}

fn part1(input: Input) -> i32 {
    let res = input.reports.iter().filter(|line| {
        check_line(line, 0)
    }).count() as i32;

    res
}


fn part2(input: Input) -> i32 {
    let res = input.reports.into_iter().filter(|line| {
        let rev_line = line.iter().rev().copied().collect::<Vec<i32>>();
        check_line(line, 1) || check_line(&rev_line, 1)
    }).count() as i32;

    res
}

fn check_line(line: &[i32], max_errors: i32) -> bool {
    let mut prev = 0;
    let mut next = 1;
    let mut prev_diff = 0;
    let mut num_errors = 0;
    while next < line.len() {
        let diff = line[next] - line[prev];
        if diff.abs() < 1 || diff.abs() > 3 || diff * prev_diff < 0 {
            if num_errors < max_errors {
                num_errors += 1;
                //skip this one, maybe next one will work
                next += 1;
                continue;
            }
            return false;
        }
        prev_diff = diff;
        prev = next;
        next += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/02.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(2, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(5, res)
    }
}
