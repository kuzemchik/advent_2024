use crate::day16::Dir::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/16.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 16");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}


#[derive(Clone)]
struct Input {
    map: Vec<Vec<char>>,
}

#[derive(Debug)]
enum InputError {}

#[derive(Clone)]
struct Deer {
    x: usize,
    y: usize,
    dir: Dir,
    score: i64,
}

impl Deer {
    fn step(&self, dir: &Dir) -> Deer {
        let mut score = self.score;
        if *dir == self.dir {
            score += 1;
        } else {
            score += 1001;
        }
        match dir {
            Up => {
                Deer {
                    x: self.x,
                    y: self.y - 1,
                    dir: dir.clone(),
                    score,
                }
            }
            Down => {
                Deer {
                    x: self.x,
                    y: self.y + 1,
                    dir: dir.clone(),
                    score,
                }
            }
            Left => {
                Deer {
                    x: self.x - 1,
                    y: self.y,
                    dir: dir.clone(),
                    score,
                }
            }
            Right => {
                Deer {
                    x: self.x + 1,
                    y: self.y,
                    dir: dir.clone(),
                    score,
                }
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input.lines().map(|l| {
            l.chars().collect()
        }).collect();
        Ok(Input {
            map
        })
    }
}

fn part1(input: Input) -> i64 {
    let map = input.map;
    let deer = Deer {
        y: map.len() - 2,
        x: 1,
        score: 0,
        dir: Right,
    };

    let mut deers = vec![];
    let mut queue = BinaryHeap::new();

    queue.push(Reverse((deer.score, deers.len())));
    deers.push(deer);

    let mut visited: HashMap<(usize, usize, Dir), usize> = HashMap::new();
    let mut best_score = 0;
    while let Some(Reverse((_, id))) = queue.pop() {
        let x = deers[id].x;
        let y = deers[id].y;
        let score = deers[id].score;
        let dir = deers[id].dir.clone();


        // println!("{}", score);
        // for my in 0..map.len() {
        //     for mx in 0..map[0].len() {
        //         if my == y && mx == x {
        //             print!("@");
        //         } else {
        //             print!("{}", map[my][mx]);
        //         }
        //     }
        //     println!()
        // }

        if map[y][x] == 'E' {
            best_score = score;
            break;
        }
        if let Some(prev) = visited.get(&(x, y, dir.clone())) {
            if deers[*prev].score <= score {
                continue;
            }
        } else {
            visited.insert((x, y, dir), id);
        }
        for dir in [Left, Right, Up, Down] {
            let new_deer = deers[id].step(&dir);
            let new_score = new_deer.score;
            if map[new_deer.y][new_deer.x] == '#' {
                continue;
            }
            let distance = new_deer.y - 1 + map[0].len() - 2 - new_deer.x;
            let id = if let Some(prev) = visited.get(&(new_deer.x, new_deer.y, dir)) {
                if deers[*prev].score < new_score {
                    continue;
                } else {
                    deers[*prev] = new_deer;
                }
                *prev
            } else {
                let id = deers.len();
                deers.push(new_deer);
                id
            };

            queue.push(Reverse((new_score + distance as i64, id)))
        }
    }
    best_score
}


fn part2(input: Input) -> i64 {
    let map = input.map;
    let deer = Deer {
        y: map.len() - 2,
        x: 1,
        score: 0,
        dir: Right,
    };

    let mut deers = vec![];
    let mut queue = BinaryHeap::new();


    queue.push(Reverse((deer.score, deers.len(), vec![0])));
    deers.push(deer);
    let mut visited_tiles: HashSet<(usize, usize)> = HashSet::new();

    let mut visited: HashMap<(usize, usize, Dir), usize> = HashMap::new();
    let mut best_score = i64::MAX;

    while let Some(Reverse((_, id, path))) = queue.pop() {
        let x = deers[id].x;
        let y = deers[id].y;
        let score = deers[id].score;
        let dir = deers[id].dir.clone();


        if map[y][x] == 'E' {
            if score <= best_score {
                let mut local_queue = VecDeque::new();
                local_queue.push_back((x, y));
                path.iter().for_each(|idx| {
                    visited_tiles.insert((deers[*idx].x, deers[*idx].y));
                });
                // println!("{}", score);
                // for my in 0..map.len() {
                //     for mx in 0..map[0].len() {
                //         if visited_tiles.contains(&(mx, my)) {
                //             print!("O");
                //         } else {
                //             print!("{}", map[my][mx]);
                //         }
                //     }
                //     println!()
                // }
                best_score = score;
            } else {
                break;
            }
        }
        if let Some(prev) = visited.get(&(x, y, dir.clone())) {
            if deers[*prev].score < score {
                continue;
            }
        } else {
            visited.insert((x, y, dir), id);
        }

        for dir in [Left, Right, Up, Down] {
            let new_deer = deers[id].step(&dir);
            let new_score = new_deer.score;
            // let new_x = new_deer.x;
            // let new_y = new_deer.y;
            if map[new_deer.y][new_deer.x] == '#' {
                continue;
            }
            // let distance = new_deer.y - 1 + map[0].len() - 2 - new_deer.x;
            let id = if let Some(prev) = visited.get(&(new_deer.x, new_deer.y, dir)) {
                if deers[*prev].score < new_score {
                    continue;
                }
                deers[*prev] = new_deer;

                *prev
            } else {
                let id = deers.len();
                deers.push(new_deer);
                id
            };
            let mut new_path = path.clone();
            new_path.push(id);

            queue.push(Reverse((new_score, id, new_path)))
        }
    }

    visited_tiles.len() as i64
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/16.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(7036, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(45, res)
    }
}
