use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/05.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 5");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Input {
    rules: Vec<Vec<u8>>,
    pages: Vec<Vec<u8>>,
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().collect::<Vec<&str>>();
        let rules_strs = lines.iter().take_while(|l| !l.is_empty()).collect::<Vec<_>>();
        let book_strs: Vec<&str> = lines[rules_strs.len() + 1..].to_vec();
        let rules: Vec<Vec<u8>> = rules_strs.iter()
            .map(|s|
                s.split('|')
                    .map(|s| u8::from_str(s).unwrap())
                    .collect()
            )
            .collect::<Vec<Vec<u8>>>();
        let pages = book_strs.iter().take_while(|l| !l.is_empty()).map(|s| {
            s.split(',').map(|s| u8::from_str(s).unwrap()).collect::<Vec<u8>>()
        }).collect::<Vec<Vec<u8>>>();
        Ok(Input {
            rules,
            pages,
        })
    }
}

fn common(input: Input) -> (i32, i32) {
    let idx = input.rules.iter().map(|rule| {
        (rule[0], rule[1])
    }).collect::<HashSet<(u8, u8)>>();
    let (correct, incorrect) = input.pages.iter().fold((0, 0), |(mut correct, mut incorrect), page| {
        let mut sorted_page = page.clone();
        sorted_page.sort_by(|l, r| {
            if idx.contains(&(*l, *r)) {
                Ordering::Less
            } else if idx.contains(&(*r, *l)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        if sorted_page.eq(page) {
            correct += page[page.len() / 2] as i32;
        } else {
            incorrect += sorted_page[sorted_page.len() / 2] as i32;
        }
        (correct, incorrect)
    });
    (correct, incorrect)
}

fn part1(input: Input) -> i32 {
    let (correct, _incorrect) = common(input);
    correct
}

fn part2(input: Input) -> i32 {
    let (_correct, incorrect) = common(input);
    incorrect
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/05.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(143, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(123, res)
    }
}
