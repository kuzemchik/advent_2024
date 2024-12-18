use crate::day15::Dir::*;
use crate::day15::Item::*;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/15.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 15");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn step(&self, dir: &Dir) -> Pos {
        match dir {
            Up => {
                Pos {
                    x: self.x,
                    y: self.y - 1,
                }
            }
            Down => {
                Pos {
                    x: self.x,
                    y: self.y + 1,
                }
            }
            Left => {
                Pos {
                    x: self.x - 1,
                    y: self.y,
                }
            }
            Right => {
                Pos {
                    x: self.x + 1,
                    y: self.y,
                }
            }
        }
    }
    fn xu(&self) -> usize {
        self.x as usize
    }

    fn yu(&self) -> usize {
        self.y as usize
    }
}


type Store = Vec<Vec<Item>>;

// impl Pos {
//     fn step(&mut self, dir: &Dir, map: &mut Store) {
//         match dir {
//             Up => {
//                 let y = self.y + 1;
//                 map[y][self.x];
//                 self.y += 1;
//             }
//         }
//     }
// }

#[derive(Clone)]
struct Input {
    map: Store,
    actions: Vec<Dir>,
    robot: Pos,
}

#[derive(Debug)]
enum InputError {}

#[derive(Clone, Eq, PartialEq)]
enum Item {
    Wall,
    Box,
    L,
    R,
    Free,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let mut robot: Pos = Pos { x: 0, y: 0 };
        let map: Vec<Vec<Item>> = parts[0].lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| {
                let item = match c {
                    '#' => Item::Wall,
                    '@' => {
                        robot = Pos {
                            x: x as i32,
                            y: y as i32,
                        };
                        Item::Free
                    }
                    'O' => Item::Box,
                    '.' => Item::Free,
                    _ => panic!("Unknown character {}", c)
                };
                item
            }).collect()
        }).collect();

        let actions: Vec<Dir> = parts[1].chars().filter(|v| {
            *v != '\n'
        }).map(|v| {
            match v {
                '<' => Left,
                '>' => Right,
                '^' => Up,
                'v' => Down,
                _ => panic!("unknown direction")
            }
        }).collect();
        Ok(Input {
            map,
            actions,
            robot,
        })
    }
}

fn part1(input: Input) -> i64 {
    let mut store = input.map;


    let mut robot = input.robot;
    for dir in input.actions {
        let next = robot.step(&dir);
        if try_push(&next, &dir, &mut store) {
            robot = next;
        }
        // for y in 0..store.len() {
        //     for x in 0..store[0].len() {
        //         if store[y][x] == Box {
        //             print!("O");
        //         } else if store[y][x] == Wall {
        //             print!("#");
        //         } else if robot.xu() == x && robot.yu() == y {
        //             print!("@");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!()
        // }
    }
    let mut res = 0;
    for y in 0..store.len() {
        for x in 0..store[0].len() {
            if store[y][x] == Box {
                res += 100 * y as i64 + x as i64;
            }
        }
    }
    res
}

fn try_push(pos: &Pos, dir: &Dir, store: &mut Store) -> bool {
    let new_pos = pos.step(dir);
    let item = store[pos.yu()][pos.xu()].clone();
    match &item {
        Wall => false,
        Box =>
            if try_push(&new_pos, dir, store) {
                store[pos.yu()][pos.xu()] = Free;
                store[new_pos.yu()][new_pos.xu()] = item;
                true
            } else {
                false
            }
        ,
        Free => true,
        _ => panic!("shouldn't happen")
    }
}

fn part2(input: Input) -> i64 {
    let mut store: Store = input.map.iter().map(|l| {
        l.iter().flat_map(|i| {
            if *i == Box {
                vec![L, R]
            } else {
                vec![i.clone(), i.clone()]
            }
        }).collect()
    }).collect();
    let mut robot = Pos {
        x: input.robot.x * 2,
        y: input.robot.y,
    };
    // for y in 0..store.len() {
    //     for x in 0..store[0].len() {
    //         if store[y][x] == L {
    //             print!("[");
    //         } else if store[y][x] == R {
    //             print!("]");
    //         } else if store[y][x] == Wall {
    //             print!("#");
    //         } else if robot.xu() == x && robot.yu() == y {
    //             print!("@");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!()
    // }


    for dir in input.actions {
        let next = robot.step(&dir);
        if let Some(path) = check_push(&next, &dir, &mut store) {
            path.iter().rev().for_each(|pos| {
                let new_pos = pos.step(&dir);
                let item = store[pos.yu()][pos.xu()].clone();
                store[pos.yu()][pos.xu()] = Free;
                store[new_pos.yu()][new_pos.xu()] = item;
            });
            robot = next;
        }
        // println!("{:?}", &dir);
        // for y in 0..store.len() {
        //     for x in 0..store[0].len() {
        //         if store[y][x] == L {
        //             print!("[");
        //         } else if store[y][x] == R {
        //             print!("]");
        //         } else if store[y][x] == Wall {
        //             print!("#");
        //         } else if robot.xu() == x && robot.yu() == y {
        //             print!("@");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!()
        // }
    }
    let mut res = 0;
    for y in 0..store.len() {
        for x in 0..store[0].len() {
            if store[y][x] == L {
                res += 100 * y as i64 + x as i64;
            }
        }
    }

    res
}

fn check_push(pos: &Pos, dir: &Dir, store: &Store) -> Option<Vec<Pos>> {
    let mut queue = VecDeque::new();

    queue.push_back(pos.clone());
    let mut path = vec![];
    let mut visited: HashSet<Pos> = HashSet::new();
    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        let item = &store[pos.yu()][pos.xu()];
        if *item == Wall {
            return None;
        } else if *item == Free {
            continue
        } else if *dir == Up || *dir == Down {
            if *item == L {
                queue.push_back(pos.step(&Right))
            } else if *item == R {
                queue.push_back(pos.step(&Left))
            }
        }
        queue.push_back(pos.step(dir));
        visited.insert(pos.clone());
        path.push(pos);
    }
    Some(path)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/15.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(10092, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(9021, res)
    }
}
