use std::fs;
use std::str::FromStr;
use regex::Regex;

pub fn print() {
    let s = fs::read_to_string("data/03.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 3");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Input {
    input: String,
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Input {
            input: input.to_string()
        })
    }
}

fn part1(input: Input) -> i32 {
    let re: Regex = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();

    re.captures_iter(input.input.as_str()).map(|cap| {
        let left = cap.get(1).and_then(|v| i32::from_str(v.as_str()).ok()).unwrap();
        let right = cap.get(2).and_then(|v| i32::from_str(v.as_str()).ok()).unwrap();
        vec![left, right]
    }).fold(0, |acc, v| { acc + v[0] * v[1] })
}


fn part2(input: Input) -> i32 {
    let re: Regex = Regex::new("(?<mul>mul\\((\\d+),(\\d+)\\))|(?<do>do\\(\\))|(?<dont>don't\\(\\))").unwrap();

    let (sum, _) = re.captures_iter(input.input.as_str()).fold((0, true), |(acc, state), cap| {
        if cap.name("do").is_some() {
            (acc, true)
        } else if cap.name("dont").is_some() {
            (acc, false)
        } else if cap.name("mul").is_some() && state {
            let left = cap.get(2).and_then(|m| i32::from_str(m.as_str()).ok()).unwrap();
            let right = cap.get(3).and_then(|m| i32::from_str(m.as_str()).ok()).unwrap();
            (acc + (left * right), state)
        } else {
            (acc, state)
        }
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/03.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(161, res)
    }

    #[test]
    fn test_part2() {
        let input = Input::from_str("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))").unwrap();
        let res = part2(input);
        assert_eq!(48, res)
    }
}
