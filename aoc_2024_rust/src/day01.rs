use std::collections::HashMap;
use std::fs;
use std::str::FromStr;


pub fn print() {
    let s = fs::read_to_string("data/01.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 1");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Input {
    left: Vec<i32>,
    right: Vec<i32>,
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut left = vec![];
        let mut right = vec![];
        for line in input.lines() {
            let items = line.split("   ").collect::<Vec<_>>();
            assert_eq!(items.len(), 2, "Error split: {}", line);

            if let Ok(value) = i32::from_str(items[0]) {
                left.push(value);
            } else {
                panic!("Error parsing left {}", line)
            }
            if let Ok(value) = i32::from_str(items[1]) {
                right.push(value);
            } else {
                panic!("Error parsing right {}", line);
            }
        }
        Ok(Input {
            left,
            right,
        })
    }
}


fn part1(mut input: Input) -> i32 {
    input.left.sort_by_key(|v| *v);
    input.right.sort_by_key(|v| *v);

    let res = input.left.iter().zip(input.right.iter()).fold(0, |acc, (l, r)| {
        acc + (*l - *r).abs()
    });

    res
}


fn part2(input: Input) -> i32 {
    let index = input.right.iter().fold(HashMap::new(), |mut acc, v| {
        if let Some(value) = acc.get(v) {
            acc.insert(v, value + 1);
        } else {
            acc.insert(v, 1);
        }
        acc
    });

    let res = input.left.iter().fold(0, |acc, v| {
        acc + v * index.get(v).copied().unwrap_or(0)
    });

    res
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/01.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(11, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(31, res)
    }
}
