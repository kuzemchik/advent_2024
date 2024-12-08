use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/08.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 8");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Input {
    raw: String,
    map: Vec<Vec<char>>,
}


#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input.lines()
            .map(|line| {
                line.chars().collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();
        Ok(Input {
            raw: input.to_string(),
            map,
        })
    }
}


#[derive(Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Eq, PartialEq, Hash)]
struct Antennae {
    signal: char,
    pos: Pos,
}


impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Pos) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

fn part1(input: Input) -> i64 {
    let max_x = input.map.len() as i32;
    let max_y = input.map[0].len() as i32;
    let mut antennas = HashMap::new();
    input.raw.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if c != '.' {
                let antennae = Antennae {
                    signal: c,
                    pos: Pos { x, y },
                };
                antennas.entry(c).or_insert(Vec::new()).push(antennae);
            }
        })
    });
    // println!("max_x {}", max_x);
    // println!("max_y {}", max_y);
    //    y1 y2 y3
    // x1 u     w
    // x2
    // x3 w     u
    let mut antipods = HashSet::new();
    antennas.iter().for_each(|(_, lst)| {
        for a in lst.iter() {
            for b in lst.iter() {
                if *a == *b || b.pos > a.pos {
                    continue;
                }
                let ax = a.pos.x as i32;
                let bx = b.pos.x as i32;
                let dx = ax - bx;
                let ay = a.pos.y as i32;
                let by = b.pos.y as i32;
                let dy = ay - by;

                let s1x = ax + dx;
                let s1y = ay + dy;
                // let mut map = input.map.clone();
                // map[a.pos.x][a.pos.y] = '*';
                // map[b.pos.x][b.pos.y] = '$';
                // println!("antipod: {:?}", (s1x, s1y));
                if s1x >= 0 && s1x < max_x && s1y >= 0 && s1y < max_y {
                    antipods.insert((s1x, s1y));
                    // map[s1x as usize][s1y as usize] = '#';
                }

                let s2x = bx - dx;
                let s2y = by - dy;
                // println!("antipod: {:?}", (s2x, s2y));
                if s2x >= 0 && s2x < max_x && s2y >= 0 && s2y < max_y {
                    antipods.insert((s2x, s2y));
                    // map[s2x as usize][s2y as usize] = '#';
                }
                // map.iter().for_each(|row| {
                //     row.iter().for_each(|c| {
                //         print!("{}", c);
                //     });
                //     println!();
                // });
            }
        }
    });
    antipods.len() as i64
}

fn part2(input: Input) -> i64 {
    let max_x = input.map.len() as i32;
    let max_y = input.map[0].len() as i32;
    let mut antennas = HashMap::new();
    input.raw.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if c != '.' {
                let antennae = Antennae {
                    signal: c,
                    pos: Pos { x, y },
                };
                antennas.entry(c).or_insert(Vec::new()).push(antennae);
            }
        })
    });
    // println!("max_x {}", max_x);
    // println!("max_y {}", max_y);
    //    y1 y2 y3
    // x1 u     w
    // x2
    // x3 w     u
    // let mut map = input.map.clone();
    let mut antipods = HashSet::new();
    antennas.iter().for_each(|(_, lst)| {
        for a in lst.iter() {
            for b in lst.iter() {
                if *a == *b || b.pos > a.pos {
                    continue;
                }
                let ax = a.pos.x as i32;
                let bx = b.pos.x as i32;
                let dx = ax - bx;
                let ay = a.pos.y as i32;
                let by = b.pos.y as i32;
                let dy = ay - by;
                antipods.insert((ax, ay));
                antipods.insert((bx, by));

                // map[a.pos.x][a.pos.y] = '*';
                // map[b.pos.x][b.pos.y] = '$';
                let mut s1x = ax;
                let mut s1y = ay;
                loop {
                    s1x += dx;
                    s1y += dy;
                    if s1x >= 0 && s1x < max_x && s1y >= 0 && s1y < max_y {
                        // println!("antipod: {:?}", (s1x, s1y));
                        antipods.insert((s1x, s1y));
                        // map[s1x as usize][s1y as usize] = '#';
                    } else {
                        break;
                    }
                }


                let mut s2x = bx;
                let mut s2y = by;
                loop {
                    s2x -= dx;
                    s2y -= dy;
                    if s2x >= 0 && s2x < max_x && s2y >= 0 && s2y < max_y {
                        // println!("antipod: {:?}", (s2x, s2y));
                        antipods.insert((s2x, s2y));
                        // map[s2x as usize][s2y as usize] = '#';
                    } else {
                        break;
                    }
                }

                // map.iter().for_each(|row| {
                //     row.iter().for_each(|c| {
                //         print!("{}", c);
                //     });
                //     println!();
                // });
            }
        }
    });
    // println!("{:?}", antipods);
    // map.iter().for_each(|row| {
    //     row.iter().for_each(|c| {
    //         print!("{}", c);
    //     });
    //     println!();
    // });
    antipods.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/08.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let s = fs::read_to_string("tests/08.txt").unwrap();
        let input = Input::from_str(s.as_str()).expect("cannot parse input");
        let res = part1(input);
        assert_eq!(14, res)
    }

    #[test]
    fn test_part1_2() {
        let s = fs::read_to_string("tests/08_2.txt").unwrap();
        let input = Input::from_str(s.as_str()).expect("cannot parse input");
        let res = part1(input);
        assert_eq!(4, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(34, res)
    }

    #[test]
    fn test_part2_2() {
        let s = fs::read_to_string("tests/08_2.txt").unwrap();
        let input = Input::from_str(s.as_str()).expect("cannot parse input");
        let res = part2(input);
        assert_eq!(8, res)
    }
}
