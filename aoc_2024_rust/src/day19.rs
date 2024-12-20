use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/19.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 19");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Input {
    towels: Vec<String>,
    patterns: Vec<String>,
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let towels: Vec<String> = parts[0].split(", ").map(|s| s.to_string()).collect();
        let patterns: Vec<String> = parts[1].lines().map(|s| s.to_string()).collect();

        Ok(Input {
            towels,
            patterns,
        })
    }
}

fn part1(input: Input) -> i64 {
    let mut num_towels = 0;
    let mut fails = HashSet::new();
    input.patterns.into_iter().for_each(|p| {
        let contained_towels = input.towels.clone().into_iter().filter(|t| {
            p.contains(t)
        }).collect::<Vec<_>>();

        if rec_match(&p, &contained_towels, &mut fails) {
            num_towels += 1;
        };
    });
    num_towels
}

fn rec_match(pattern: &str, towels: &Vec<String>, fails: &mut HashSet<String>) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if fails.contains(pattern) {
        return false;
    }
    for towel in towels.iter() {
        if pattern.starts_with(towel) && rec_match(&pattern[towel.len()..], towels, fails) {
            return true;
        }
    }
    fails.insert(pattern.to_string());
    false
}

fn part2(input: Input) -> i64 {
    let mut num_towels = 0;
    let mut patterns = HashMap::new();
    input.patterns.into_iter().for_each(|p| {
        let contained_towels = input.towels.clone().into_iter().filter(|t| {
            p.contains(t)
        }).collect::<Vec<_>>();
        
        num_towels += rec_match_count(&p, &contained_towels, &mut patterns);
    });
    num_towels
}

fn rec_match_count(pattern: &str, towels: &Vec<String>, patterns: &mut HashMap<String, i64>) -> i64 {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(n) = patterns.get(pattern) {
        return *n;
    }
    let mut n = 0;
    for towel in towels.iter() {
        if pattern.starts_with(towel) {
            n += rec_match_count(&pattern[towel.len()..], towels, patterns)
        }
    }
    patterns.insert(pattern.to_string(), n);

    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn input() -> Input {
        let s = fs::read_to_string("tests/19.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }

    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(6, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(16, res)
    }
}
