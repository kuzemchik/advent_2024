use std::collections::{HashSet, VecDeque};
use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/10.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 10");
    let (part1, part2) = parts(input);
    println!("Part 1 {}", part1);
    println!("Part 2 {}", part2);
}

#[derive(Clone)]
struct Input {
    map: Vec<Vec<u8>>,
    starts: Vec<Pos>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Pos {
    idx: i32,
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut starts = vec![];
        let mut idx = 0;
        let map: Vec<Vec<u8>> = input.lines().enumerate().map(|(x, line)|
            line.chars().enumerate().map(|(y, c)| {
                let height = u8::from_str(String::from(c).as_str()).expect(format!("cannot parse number: {}", c).as_str());
                if height == 0 {
                    starts.push(Pos {
                        idx,
                        x,
                        y,
                    });
                    idx += 1;
                }
                height
            }).collect()
        ).collect();

        Ok(Input {
            map,
            starts,
        })
    }
}

fn parts(input: Input) -> (i64, i64) {
    let map = &input.map;
    let mut queue = VecDeque::new();
    queue.extend(input.starts.iter().cloned());
    let mut trails = HashSet::new();
    let mut num_trails = 0;
    let max_x = map.len() - 1;
    let max_y = map[0].len() - 1;
    while let Some(pos) = queue.pop_front() {
        let value = map[pos.x][pos.y];
        if value == 9 {
            trails.insert(pos);
            num_trails += 1;
            continue;
        }

        if pos.x > 0 && map[pos.x - 1][pos.y] == value + 1 {
            queue.push_back(
                Pos {
                    idx: pos.idx,
                    x: pos.x - 1,
                    y: pos.y,
                }
            )
        }
        if pos.y > 0 && map[pos.x][pos.y - 1] == value + 1 {
            queue.push_back(
                Pos {
                    idx: pos.idx,
                    x: pos.x,
                    y: pos.y - 1,
                }
            )
        }
        if pos.x < max_x && map[pos.x + 1][pos.y] == value + 1 {
            queue.push_back(
                Pos {
                    idx: pos.idx,
                    x: pos.x + 1,
                    y: pos.y,
                }
            )
        }
        if pos.y < max_y && map[pos.x][pos.y + 1] == value + 1 {
            queue.push_back(
                Pos {
                    idx: pos.idx,
                    x: pos.x,
                    y: pos.y + 1,
                }
            )
        }
    }
    (trails.len() as i64, num_trails as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/10.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_parts() {
        let (heads, total) = parts(input());
        assert_eq!(36, heads);
        assert_eq!(81, total);
    }
}
