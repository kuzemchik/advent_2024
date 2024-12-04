use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/04.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 4");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Input {
    map: Vec<Vec<char>>,
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input.lines().map(|l| l.chars().collect()).collect();
        Ok(Input {
            map
        })
    }
}

fn part1(input: Input) -> i32 {
    let paths = [
        vec![(0, 1), (0, 2), (0, 3)],
        vec![(0, -1), (0, -2), (0, -3)],
        vec![(1, 0), (2, 0), (3, 0)],
        vec![(-1, 0), (-2, 0), (-3, 0)],
        vec![(1, 1), (2, 2), (3, 3)],
        vec![(1, -1), (2, -2), (3, -3)],
        vec![(-1, 1), (-2, 2), (-3, 3)],
        vec![(-1, -1), (-2, -2), (-3, -3)],
    ];
    // let mut starts: Vec<(usize, usize)> = vec![];
    let x_max = input.map.len();
    let y_max = input.map[0].len();
    let chars = "XMAS".chars().collect::<Vec<_>>();
    let mut num_found = 0;
    input.map.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, letter)| {
            if *letter == 'X' {
                // starts.push((x, y));
                paths.iter().for_each(|path| {
                    let mut matching = true;
                    let mut idx = 0;
                    while matching && idx < path.len() {
                        let (x_i, y_i) = path[idx];
                        let x = x as i32 + x_i;
                        let y = y as i32 + y_i;
                        if x >= 0 && y >= 0
                            && x < x_max as i32 && y < y_max as i32
                            && input.map[x as usize][y as usize] == chars[idx + 1]
                        {
                            matching = true;
                        } else {
                            matching = false;
                            break;
                        }
                        idx += 1;
                    }
                    if matching {
                        num_found += 1;
                    }
                })
            }
        })
    });
    num_found
}


fn part2(input: Input) -> i32 {
    let paths = [
        vec![(-1, -1), (0, 0), (1, 1)],
        vec![(1, -1), (0, 0), (-1, 1)],
        vec![(-1, 1), (0, 0), (1, -1)],
        vec![(1, 1), (0, 0), (-1, -1)],
    ];
    // let mut starts: Vec<(usize, usize)> = vec![];
    let x_max = input.map.len();
    let y_max = input.map[0].len();
    let chars = "MAS".chars().collect::<Vec<_>>();
    let mut num_found = 0;
    input.map.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, letter)| {
            if *letter == 'A' {
                // starts.push((x, y));
                let all_match = paths.iter().filter(|path| {
                    let mut matching = true;
                    let mut idx = 0;
                    let mut str = String::new();
                    while matching && idx < path.len() {
                        let (x_i, y_i) = path[idx];
                        let x = x as i32 + x_i;
                        let y = y as i32 + y_i;
                        if x >= 0 && y >= 0
                            && x < x_max as i32 && y < y_max as i32
                            && input.map[x as usize][y as usize] == chars[idx]
                        {
                            matching = true;
                            str.push(input.map[x as usize][y as usize]);
                        } else {
                            matching = false;
                            break;
                        }

                        idx += 1;
                    }
                    matching
                }).count() > 1;
                if all_match {
                    num_found += 1;
                }
            }
        })
    });
    num_found
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/04.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(18, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(9, res)
    }
}
