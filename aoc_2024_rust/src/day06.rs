use crate::day06::Dir::*;
use crate::day06::StepResult::{Edge, Step};
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/06.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 6");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Input {
    map: Vec<Vec<char>>,
    start: (usize, usize),
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut start = (0, 0);
        let map = input.lines().enumerate()
            .map(|(x, line)| {
                line.chars().enumerate().map(|(y, c)| {
                    if c == '^' {
                        start = (x, y)
                    }
                    c
                }).collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        Ok(Input {
            map,
            start,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

enum StepResult {
    Edge,
    Step(usize, usize),
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
struct Pos {
    x: usize,
    y: usize,
    dir: Dir,
}


fn part1(input: Input) -> i32 {
    let mut map = input.map.clone();
    let x_max = map.len() - 1;
    let y_max = map[0].len() - 1;
    let mut curr_pos = Pos {
        x: input.start.0,
        y: input.start.1,
        dir: Up,
    };

    let mut num_visited = 1;
    while let Step(x_next, y_next) = step(&curr_pos, x_max, y_max) {
        if map[x_next][y_next] == '#' {
            curr_pos = turn(&curr_pos);
        } else {
            if map[x_next][y_next] == '.' {
                num_visited += 1;
            }
            map[curr_pos.x][curr_pos.y] = '*';
            curr_pos = Pos {
                x: x_next,
                y: y_next,
                dir: curr_pos.dir,
            }
        }
    }

    num_visited
}


fn step(pos: &Pos, x_max: usize, y_max: usize) -> StepResult {
    let Pos { x, y, dir } = pos.clone();
    match dir {
        Up => if x > 0 {
            Step(x - 1, y)
        } else { Edge },
        Right => if y < y_max {
            Step(x, y + 1)
        } else { Edge },
        Down => if x < x_max {
            Step(x + 1, y)
        } else { Edge },
        Left => if y > 0 {
            Step(x, y - 1)
        } else { Edge }
    }
}

fn turn(pos: &Pos) -> Pos {
    let mut new_pos = pos.clone();
    let new_dir = match new_pos.dir {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up
    };
    new_pos.dir = new_dir;
    new_pos
}


fn loops(pos: &Pos, map: &Vec<Vec<char>>) -> bool {
    let x_max = map.len() - 1;
    let y_max = map[0].len() - 1;
    let mut curr_pos = pos.clone();
    let mut path: Vec<Pos> = vec![];
    while let Step(x_next, y_next) = step(&curr_pos, x_max, y_max) {
        if map[x_next][y_next] == '#' {
            if path.iter().any(|p| p.eq(&curr_pos)) {
                return true;
            }
            let turned = turn(&curr_pos);
            path.push(curr_pos);
            curr_pos = turned;
        } else {
            curr_pos = Pos {
                x: x_next,
                y: y_next,
                dir: curr_pos.dir,
            }
        }
    }
    false
}
fn part2(input: Input) -> i32 {
    let mut map = input.map.clone();
    let x_max = map.len() - 1;
    let y_max = map[0].len() - 1;
    let mut curr_pos = Pos {
        x: input.start.0,
        y: input.start.1,
        dir: Up,
    };

    let mut num_loops = 0;
    let mut new_stones = HashSet::new();
    while let Step(x_next, y_next) = step(&curr_pos, x_max, y_max) {
        if map[x_next][y_next] == '#' {
            curr_pos = turn(&curr_pos);
        } else {
            if map[x_next][y_next] == '.' {
                map[x_next][y_next] = '#';
                if loops(&curr_pos, &map) {
                    num_loops += 1;
                    new_stones.insert((x_next, y_next));
                }
                map[x_next][y_next] = '.';
            }
            map[curr_pos.x][curr_pos.y] = '*';
            curr_pos = Pos {
                x: x_next,
                y: y_next,
                dir: curr_pos.dir,
            }
        }
    }

    println!("loops: {}", num_loops);
    new_stones.len() as i32
}
//
// fn print_stones(map: &Vec<Vec<char>>, stones: &HashSet<(usize, usize)>) {
//     let mut curr_map = map.clone();
//     stones.iter().for_each(|(x_next, y_next)| {
//         curr_map[*x_next][*y_next] = 'O';
//     });
//     println!("New Path");
//     curr_map.iter().for_each(|row| {
//         row.iter().for_each(|c| {
//             print!("{}", c);
//         });
//         println!();
//     });
// }
//
//
// fn next_stone(pos: &Pos,
//               stones: &Vec<(usize, usize)>) -> Option<Pos> {
//     let Pos { x, y, dir } = pos;
//     match dir {
//         Up => {
//             stones.iter().filter(|(stone_x, stone_y)| {
//                 *stone_x < *x && *stone_y == *y
//             }).max_by_key(|(x, _)| x).copied()
//                 .map(|(x, y)| Pos { x: x + 1, y: y, dir: dir.clone() })
//         }
//         Left => {
//             stones.iter().filter(|(stone_x, stone_y)| {
//                 *stone_x == *x && *stone_y < *y
//             }).max_by_key(|(_, y)| y).copied()
//                 .map(|(x, y)| Pos { x: x, y: y + 1, dir: dir.clone() })
//         }
//         Right => {
//             stones.iter().filter(|(stone_x, stone_y)| {
//                 *stone_x == *x && *stone_y > *y
//             }).min_by_key(|(_, y)| y).copied()
//                 .map(|(x, y)| Pos { x: x, y: y - 1, dir: dir.clone() })
//         }
//         Down => {
//             stones.iter().filter(|(stone_x, stone_y)| {
//                 *stone_x > *x && *stone_y == *y
//             }).min_by_key(|(x, _)| x).copied()
//                 .map(|(x, y)| Pos { x: x - 1, y: y, dir: dir.clone() })
//         }
//     }
// }
//
// fn check_loop(start: &Pos, stones: &Vec<(usize, usize)>) -> Vec<Pos> {
//     let mut next = start.clone();
//     let mut path: Vec<Pos> = vec![];
//     while let Some(stone) = next_stone(&next, stones) {
//         if path.iter().any(|p| p.eq(&stone)) {
//             return path;
//         }
//         next = turn(&stone);
//         path.push(stone);
//     }
//     vec![]
// }
// // Actually slower
// fn part2_skip_stones(input: Input) -> i32 {
//     let mut map = input.map.clone();
//     let x_max = map.len() - 1;
//     let y_max = map[0].len() - 1;
//
//     let mut stones = map.iter().enumerate().flat_map(|(x, row)| {
//         let result = row.iter().enumerate()
//             .filter(|(_, &c)| c == '#')
//             .map(|(y, _)| y)
//             .map(move |y| {
//                 (x, y)
//             });
//         result
//     }).collect::<Vec<(usize, usize)>>();
//
//
//     let mut curr_pos = Pos {
//         x: input.start.0,
//         y: input.start.1,
//         dir: Up,
//     };
//     let mut new_stones = HashSet::new();
//     while let Step(x_next, y_next) = step(&curr_pos, x_max, y_max) {
//         let turned = turn(&curr_pos);
//         if map[x_next][y_next] == '#' {
//             curr_pos = turned;
//         } else {
//             if map[x_next][y_next] == '.' {
//                 if let Some(pos) = next_stone(&turned, &stones) {
//                     let stone = (x_next, y_next);
//                     if !new_stones.contains(&stone) && !stone.eq(&input.start) {
//                         stones.push(stone);
//                         let path = check_loop(&turned, &stones);
//                         let stone = stones.pop().unwrap();
//                         if !path.is_empty() {
//                             new_stones.insert(stone);
//                         }
//                     }
//                 }
//             }
//             map[curr_pos.x][curr_pos.y] = '*';
//
//             curr_pos = Pos {
//                 x: x_next,
//                 y: y_next,
//                 dir: curr_pos.dir,
//             }
//         }
//     }
//     new_stones.len() as i32
// }


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn input() -> Input {
        let s = fs::read_to_string("tests/06.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let num_visited = part1(input());
        assert_eq!(41, num_visited)
    }

    #[test]
    fn test_part2() {
        let num_stones = part2(input());
        assert_eq!(6, num_stones)
    }

    #[test]
    fn test_part2_1() {
        let s = fs::read_to_string("tests/06_1.txt").unwrap();
        let input = Input::from_str(s.as_str()).expect("cannot parse input");
        let num_stones = part2(input);
        assert_eq!(1, num_stones)
    }

    #[test]
    fn test_part2_2() {
        let s = fs::read_to_string("tests/06_2.txt").unwrap();
        let input = Input::from_str(s.as_str()).expect("cannot parse input");
        let num_stones = part2(input);
        assert_eq!(2, num_stones)
    }
    #[test]
    fn test_part2_3() {
        let s = fs::read_to_string("tests/06_3.txt").unwrap();
        let input = Input::from_str(s.as_str()).expect("cannot parse input");
        let num_stones = part2(input);
        assert_eq!(2, num_stones)
    }

    #[test]
    fn test_part2_4() {
        let s = fs::read_to_string("tests/06_4.txt").unwrap();
        let input = Input::from_str(s.as_str()).expect("cannot parse input");
        let num_stones = part2(input);
        assert_eq!(5, num_stones)
    }
}
