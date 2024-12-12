use crate::day12::Dir::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/12.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 12");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

type HikeMap = Vec<Vec<char>>;
#[derive(Clone)]
struct Input {
    map: HikeMap,
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map: HikeMap = input.lines().map(|line| {
            line.chars().collect()
        }).collect();
        Ok(Input {
            map
        })
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Pos {
    idx: u32,
    x: usize,
    y: usize,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Dir {
    R,
    RB,
    B,
    BL,
    L,
    LT,
    T,
    TR,
}

impl Dir {
    fn sides() -> [Dir; 4] {
        [R, B, L, T]
    }

    fn corners() -> [Dir; 4] {
        [RB, BL, LT, TR, ]
    }

    fn corner_neighbors(&self) -> Vec<Dir> {
        match self {
            RB => vec![R, B],
            BL => vec![B, L],
            LT => vec![L, T],
            TR => vec![T, R],
            _ => panic!("{:?} not a corner", &self)
        }
    }
    fn offset(&self) -> (i32, i32) {
        match self {
            R => (0, 1),
            RB => (1, 1),
            B => (1, 0),
            BL => (1, -1),
            L => (0, -1),
            LT => (-1, -1),
            T => (-1, 0),
            TR => (-1, 1),
        }
    }
}
impl Pos {
    fn value(&self, map: &HikeMap) -> char {
        map[self.x][self.y]
    }

    fn get_dir(&self, dir: &Dir, map: &HikeMap) -> Option<Pos> {
        let x_max = map.len() - 1;
        let y_max = map[0].len() - 1;
        let (x, y) = dir.offset();

        let (next_x, next_y) = (self.x as i32 + x, self.y as i32 + y);

        if next_x >= 0 && next_x <= x_max as i32 && next_y >= 0 && next_y <= y_max as i32 {
            let pos = Pos {
                idx: self.idx,
                x: next_x as usize,
                y: next_y as usize,
            };
            Some(pos)
        } else {
            None
        }
    }

    fn sides(&self, map: &HikeMap) -> Vec<Option<Pos>> {
        Dir::sides().iter().map(|dir| {
            self.get_dir(dir, map)
        }).collect()
    }

    fn num_corners(&self, map: &HikeMap) -> u32 {
        let mut num_corners = 0;
        for dir in Dir::corners() {
            let value =
                self.get_dir(&dir, map)
                    .map(|v| v.value(map));

            let neighbors = dir.corner_neighbors();

            let one =
                self.get_dir(&neighbors[0], map)
                    .map(|v| v.value(map))
                    .filter(|v| *v == self.value(map));

            let another =
                self.get_dir(&neighbors[1], map)
                    .map(|v| v.value(map))
                    .filter(|v| *v == self.value(map))
                ;

            if one.is_none() && another.is_none() || (one == another && one != value) {
                num_corners += 1;
            }
        };
        num_corners
    }
}

fn part1(input: Input) -> i64 {
    let map = input.map;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut metrics: HashMap<u32, (u32, u32)> = HashMap::new();

    let mut idx = 0;
    let mut queue = VecDeque::new();

    queue.push_back(Pos { idx, x: 0, y: 0 });

    while let Some(curr) = queue.pop_front() {
        if visited.contains(&(curr.x, curr.y)) {
            continue;
        }
        let mut curr_per = 0;
        for next in curr.sides(&map) {
            if let Some(pos) = next {
                if visited.contains(&(curr.x, curr.y)) {
                    continue;
                }
                if map[pos.x][pos.y] == map[curr.x][curr.y] {
                    queue.push_front(pos);
                } else {
                    idx += 1;
                    let new_pos = Pos {
                        idx,
                        x: pos.x,
                        y: pos.y,
                    };
                    queue.push_back(new_pos);

                    curr_per += 1;
                }
            } else {
                curr_per += 1;
            }
        }
        metrics.entry(curr.idx).and_modify(|(p, s)| {
            *p += curr_per;
            *s += 1;
        }).or_insert((curr_per, 1));
        visited.insert((curr.x, curr.y));
    }

    metrics.iter().fold(0, |acc, (_id, (per, sqr))| {
        let cost = per * sqr;
        acc + (cost as i64)
    })
}


fn part2(input: Input) -> i64 {
    let map = input.map;
    let mut visited: HashMap<(usize, usize), u32> = HashMap::new();
    let mut metrics: HashMap<u32, (u32, u32)> = HashMap::new();
    let mut idx = 0;
    let mut queue = VecDeque::new();

    queue.push_back(Pos { idx, x: 0, y: 0 });

    while let Some(curr) = queue.pop_front() {
        if visited.contains_key(&(curr.x, curr.y)) {
            continue;
        }
        for pos in curr.sides(&map).into_iter().flatten() {
            if visited.contains_key(&(curr.x, curr.y)) {
                continue;
            }
            if map[pos.x][pos.y] != map[curr.x][curr.y] {
                idx += 1;
                let new_pos = Pos {
                    idx,
                    x: pos.x,
                    y: pos.y,
                };
                queue.push_back(new_pos);
            } else {
                queue.push_front(pos);
            }
        }
        let num_corners = curr.num_corners(&map);

        metrics.entry(curr.idx).and_modify(|(p, s)| {
            *s += 1;
            *p += num_corners;
        }).or_insert((num_corners, 1));
        visited.insert((curr.x, curr.y), curr.idx);
    }

    metrics.iter().fold(0, |acc, (_id, (per, sqr))| {
        let cost = per * sqr;
        acc + (cost as i64)
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/12.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(1930, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(1206, res)
    }

    #[test]
    fn test_part2_1() {
        let s = fs::read_to_string("tests/12_1.txt").unwrap();
        let input = Input::from_str(s.as_str()).expect("cannot parse input");
        let res = part2(input);
        assert_eq!(368, res)
    }

    #[test]
    fn test_part2_2() {
        let s = fs::read_to_string("tests/12_2.txt").unwrap();
        let input = Input::from_str(s.as_str()).expect("cannot parse input");
        let res = part2(input);
        assert_eq!(50, res)
    }
}
