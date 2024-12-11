use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
pub fn print() {
    let s = fs::read_to_string("data/11.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 11");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Input {
    data: Vec<u64>,
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data: Vec<u64> = input.trim().split(' ').map(|v| v.to_string()).map(|v| {
            u64::from_str(v.as_str()).expect("cannot parse value")
        }).collect();
        Ok(Input {
            data
        })
    }
}


fn part1(input: Input) -> i64 {
    let mut index = HashMap::<(u64, u16), i64>::new();

    let max_idx = 25;
    input.data.iter().fold(0, |acc, &value| {
        acc + calculate_rec(value, 0, max_idx, &mut index)
    })
}


fn part2(input: Input) -> i64 {
    let mut index = HashMap::<(u64, u16), i64>::new();

    let max_idx = 75;
    input.data.iter().fold(0, |acc, &value| {
        acc + calculate_rec(value, 0, max_idx, &mut index)
    })
}
fn calculate_rec(curr: u64, idx: u16, max_idx: u16, index: &mut HashMap<(u64, u16), i64>) -> i64 {
    let mut num_found = 0;
    if idx == max_idx {
        return 1;
    }
    if let Some(&value) = index.get(&(curr, max_idx - idx)) {
        return value;
    }
    if curr == 0 {
        num_found += calculate_rec(1, idx + 1, max_idx, index);
    } else if curr.to_string().len() % 2 == 0 {
        let len = curr.to_string().len() as u32;

        let scale: u64 = u64::pow(10, len / 2);
        let rem = curr % scale;
        num_found += calculate_rec(rem, idx + 1, max_idx, index);
        num_found += calculate_rec((curr - rem) / scale, idx + 1, max_idx, index);
    } else {
        num_found += calculate_rec(curr * 2024, idx + 1, max_idx, index);
    }
    index.insert((curr, max_idx - idx), num_found);
    num_found
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/11.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(55312, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(65601038650482, res)
    }
}
