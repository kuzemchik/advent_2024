use crate::day18::Dir::*;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::hash::Hash;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/18.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 18");
    println!("Part 1 {}", part1(input.clone(), 1024, 70));
    println!("Part 2 {}", part2(input.clone(), 70).unwrap());
}

#[derive(Clone)]
struct Input {
    blocks: Vec<(Pos, u64)>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Dir {
    Left,
    Top,
    Right,
    Down,
}

impl Dir {
    fn get_offset(&self) -> (i32, i32) {
        match self {
            Left => (-1, 0),
            Top => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
        }
    }
}

impl Pos {
    fn next(&self, x_max: i32, y_max: i32) -> Vec<Pos> {
        [Left, Top, Right, Down]
            .iter()
            .filter_map(|dir| {
                let (x_offset, y_offset) = dir.get_offset();
                let x = self.x + x_offset;
                let y = self.y + y_offset;
                if x >= 0 && x <= x_max && y >= 0 && y <= y_max {
                    Some(Pos { x, y })
                } else {
                    None
                }
            })
            .collect()
    }
}
impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        // reversed
        other.x.cmp(&self.x).then(other.y.cmp(&self.y))
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let blocks = input
            .lines()
            .enumerate()
            .map(|(z, l)| {
                let coords: Vec<&str> = l.split(",").collect();
                let x = i32::from_str(coords[0]).expect("cannot parse x");
                let y = i32::from_str(coords[1]).expect("cannot parse y");
                let pos = Pos { x, y };
                (pos, (z + 1) as u64)
            })
            .collect();
        Ok(Input { blocks })
    }
}

fn part1(input: Input, steps: u64, grid_size: i32) -> i64 {
    let x_max = grid_size;
    let y_max = grid_size;
    let start = Pos { x: 0, y: 0 };
    let target = Pos { x: x_max, y: y_max };
    let mut map: Vec<Vec<Option<u64>>> =
        vec![vec![None; (x_max + 1) as usize]; (y_max + 1) as usize];
    input.blocks.iter().for_each(|(b, ts)| {
        map[b.y as usize][b.x as usize] = Some(*ts);
    });
    if let Some((min_path, _)) = find_path(&start, &target, &map, steps, grid_size, grid_size) {
        min_path as i64
    } else {
        panic!("cannot find path!!!")
    }
}

fn part2(input: Input, grid_size: i32) -> Option<Pos> {
    let x_max = grid_size;
    let y_max = grid_size;
    let start = Pos { x: 0, y: 0 };
    let target = Pos { x: x_max, y: y_max };

    let blocks = input.blocks;
    // let map = input.map;
    let mut map: Vec<Vec<Option<u64>>> =
        vec![vec![None; (x_max + 1) as usize]; (y_max + 1) as usize];
    blocks.iter().for_each(|(b, ts)| {
        map[b.y as usize][b.x as usize] = Some(*ts);
    });

    {
        let mut last_path = vec![];
        for (block, blocks_ts) in blocks {
            if !last_path.is_empty() && !last_path.contains(&block) {
                continue;
            }

            if let Some((_min_len, min_path)) =
                find_path(&start, &target, &map, blocks_ts, grid_size, grid_size)
            {
                last_path = min_path;
            } else {
                return Some(block);
            }
        }
        None
    }

    // {
    //     let mut block_idx = 0;
    //     let mut blocks_ts = &blocks[block_idx].1;
    //
    //     let mut states: Vec<(u64, Pos, Vec<Pos>)> = vec![];
    //
    //     states.push((0, start.clone(), vec![start.clone()]));
    //     let mut queue: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
    //
    //     queue.push(Reverse((grid_size as u64 * 2, 0)));
    //
    //     let mut backlog: Vec<Reverse<(u64, usize)>> = vec![];
    //     let mut visited: HashMap<Pos, u64> = HashMap::new();
    //
    //     while let Some(Reverse((score, idx))) = queue.pop() {
    //         let next_positions = {
    //             let (curr_ts, curr, path) = &states[idx];
    //             if let Some(&block_ts) = map.get(&curr) {
    //                 if block_ts <= *blocks_ts {
    //                     continue;
    //                 }
    //             }
    //             // println!("[{}] {:?}", curr_ts, curr);
    //             // for y in 0..=y_max {
    //             //     for x in 0..=x_max {
    //             //         let cell = Pos { x, y };
    //             //         if let Some(&block) = map.get(&cell) {
    //             //             if block <= *blocks_ts {
    //             //                 print!("#");
    //             //                 continue;
    //             //             }
    //             //         }
    //             //         if path.contains(&cell) {
    //             //             print!("O");
    //             //         } else {
    //             //             print!(".");
    //             //         }
    //             //     }
    //             //     println!();
    //             // }
    //             if *curr == target {
    //                 // println!("{:?}", path);
    //                 // path.iter().all(|p| {
    //                 //     map.get(p)
    //                 //         .filter(|&block_ts| block_ts <= blocks_ts)
    //                 //         .is_none()
    //                 // })
    //                 let mut block = &blocks[block_idx].0;
    //                 while !path.contains(block) {
    //                     block_idx += 1;
    //                     if block_idx >= blocks.len() {
    //                         return None;
    //                     }
    //                     block = &blocks[block_idx].0;
    //                     blocks_ts = &blocks[block_idx].1;
    //                 }
    //                 visited = HashMap::new();
    //                 // for step in path.iter().rev() {
    //                 //     visited.remove(step);
    //                 //     if step == block {
    //                 //         // println!("step cleaned");
    //                 //         break;
    //                 //     }
    //                 // }
    //                 while let Some(Reverse((score, idx))) = backlog.pop() {
    //                     let (_, _, path) = &states[idx];
    //                     let is_blocked = path.iter().all(|p| {
    //                         map.get(p)
    //                             .filter(|&block_ts| block_ts <= blocks_ts)
    //                             .is_none()
    //                     });
    //                     if !is_blocked {
    //                         queue.push(Reverse((score, idx)));
    //                     }
    //                 }
    //                 // println!("found number");
    //                 continue;
    //             }
    //
    //             if let Some(last) = visited.get_mut(&curr) {
    //                 if *last <= *curr_ts {
    //                     backlog.push(Reverse((score, idx)));
    //                     continue;
    //                 } else {
    //                     *last = *curr_ts;
    //                 }
    //             } else {
    //                 visited.insert(curr.clone(), *curr_ts);
    //             }
    //             let next: Vec<Pos> = curr
    //                 .next(x_max, y_max)
    //                 .into_iter()
    //                 .filter(|p| !path.contains(p))
    //                 .collect();
    //
    //             (*curr_ts, next, path.clone())
    //         };
    //
    //         for next in next_positions.1 {
    //             let next_ts = next_positions.0 + 1;
    //             if next_positions.2.contains(&next) {
    //                 continue;
    //             }
    //             if let Some(&block_ts) = map.get(&next) {
    //                 if block_ts <= *blocks_ts {
    //                     continue;
    //                 }
    //             }
    //             let score = (x_max - &next.x + y_max - &next.y) as u64 + next_ts;
    //             let mut new_path = next_positions.2.clone();
    //             let idx = states.len();
    //             new_path.push(next.clone());
    //             states.push((next_ts, next.clone(), new_path));
    //             if let Some(&last) = visited.get(&next) {
    //                 if last <= next_ts {
    //                     backlog.push(Reverse((score, idx)));
    //                     continue;
    //                 }
    //             }
    //             queue.push(Reverse((score, idx)))
    //         }
    //     }
    //     Some(blocks[block_idx].0.clone())
    // }
}

fn find_path(
    start: &Pos,
    target: &Pos,
    blocks: &Vec<Vec<Option<u64>>>,
    blocks_ts: u64,
    x_max: i32,
    y_max: i32,
) -> Option<(u64, Vec<Pos>)> {
    let mut queue: BinaryHeap<Reverse<(u64, Pos)>> = BinaryHeap::new();

    let mut states = vec![vec![u64::MAX; (x_max + 1) as usize]; (y_max + 1) as usize];
    states[0][0] = 0;
    let mut paths: Vec<Vec<Option<Pos>>> =
        vec![vec![None; (x_max + 1) as usize]; (y_max + 1) as usize];
    queue.push(Reverse(((x_max + y_max) as u64, start.clone())));
    while let Some(Reverse((_score, curr))) = queue.pop() {
        let curr_ts = states[curr.y as usize][curr.x as usize];
        // println!("[{}] {:?}", curr_ts, curr);
        // for y in 0..=y_max {
        //     for x in 0..=x_max {
        //         let cell = Pos {
        //             x,
        //             y,
        //         };
        //         if let Some(&block) = blocks.get(&cell) {
        //             if block < blocks_ts {
        //                 print!("#");
        //                 continue;
        //             }
        //         }
        //         if path.contains(&cell) {
        //             print!("O");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }

        if curr == *target {
            let mut final_path = vec![];
            final_path.push(curr.clone());
            let mut next = &final_path[final_path.len() - 1];
            while let Some(prev) = &paths[next.y as usize][next.x as usize] {
                final_path.push(prev.clone());
                next = prev;
            }
            final_path.reverse();
            return Some((curr_ts, final_path));
        }

        let next_ts = curr_ts + 1;
        for next in curr.next(x_max, y_max) {
            if let Some(block_ts) = blocks[next.y as usize][next.x as usize] {
                if block_ts <= blocks_ts {
                    continue;
                }
            }

            if next_ts >= states[next.y as usize][next.x as usize] {
                continue;
            }
            states[next.y as usize][next.x as usize] = next_ts;

            let distance = x_max - next.x + y_max - next.y;
            let score = distance as u64 + next_ts;
            paths[next.y as usize][next.x as usize] = Some(curr.clone());
            // path.insert(next.clone(), curr.clone());

            queue.push(Reverse((score, next)));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn input() -> Input {
        let s = fs::read_to_string("tests/18.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }

    #[test]
    fn test_part1() {
        let res = part1(input(), 12, 6);
        assert_eq!(22, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input(), 6);
        assert_eq!(Some(Pos { x: 6, y: 1 }), res)
    }
}
