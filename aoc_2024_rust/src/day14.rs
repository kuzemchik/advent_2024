use regex::Regex;
use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/14.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 14");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Clone)]
struct Robot {
    pos: Pos,
    vel: Pos,
}

impl Robot {
    fn step(&mut self, n: i64) {
        self.pos.x += self.vel.x * n;
        self.pos.y += self.vel.y * n;
    }
}

#[derive(Clone)]
struct Input {
    robots: Vec<Robot>,
}

#[derive(Debug)]
enum InputError {}


impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut robots = vec![];
        let re: Regex = Regex::new("p=(?<x>-?\\d+),(?<y>-?\\d+) v=(?<vx>-?\\d+),(?<vy>-?\\d+)").unwrap();
        for line in input.lines() {
            re.captures(line).iter().for_each(|c| {
                let x = i64::from_str(
                    c.name("x").expect("cannot parse sign").as_str()
                ).expect("cannot parse x");
                let y = i64::from_str(
                    c.name("y").expect("cannot parse sign").as_str()
                ).expect("cannot parse y");

                let vx = i64::from_str(
                    c.name("vx").expect("cannot parse sign").as_str()
                ).expect("cannot parse vx");
                let vy = i64::from_str(
                    c.name("vy").expect("cannot parse sign").as_str()
                ).expect("cannot parse vy");
                robots.push(
                    Robot {
                        pos: Pos {
                            x,
                            y,
                        },
                        vel: Pos {
                            x: vx,
                            y: vy,
                        },
                    }
                )
            });
        };
        Ok(Input {
            robots
        })
    }
}

fn part1(input: Input) -> i64 {
    let x_max = 101;
    let y_max = 103;
    let x_mid = x_max / 2;
    let y_mid = y_max / 2;
    let mut qrant = [0; 4];
    for mut robot in input.robots.clone() {
        robot.step(100);
        let mut x = robot.pos.x % x_max;
        if x < 0 {
            x += x_max;
        }
        let mut y = robot.pos.y % y_max;
        if y < 0 {
            y += y_max;
        }
        if x < x_mid / 2 && y < y_mid {
            qrant[0] += 1;
        }
        if x > x_mid && y < y_mid {
            qrant[1] += 1;
        }
        if x > x_mid && y > y_mid {
            qrant[2] += 1;
        }
        if x < x_mid && y > y_mid {
            qrant[3] += 1;
        }
    }
    let res = qrant.iter().sum::<i64>();
    res
}

fn part2(input: Input) -> i64 {
    let x_max = 101;
    let y_max = 103;

    let mut robots = input.robots.clone();
    let mut step = 0;

    loop {
        step += 1;
        let mut found_tree = true;
        let mut map = [[false; 101]; 103];
        for robot in &mut robots {
            robot.step(1);
            let mut x = robot.pos.x % x_max;
            if x < 0 {
                x += x_max;
            }
            let mut y = robot.pos.y % y_max;
            if y < 0 {
                y += y_max;
            }
            if map[y as usize][x as usize] {
                found_tree = false;
            }
            if found_tree {
                map[y as usize][x as usize] = true;
            }
        }
        if found_tree {
            // println!("Num Steps: {}", step);
            // for y in 0..y_max {
            //     for x in 0..x_max {
            //         if map[y as usize][x as usize] {
            //             print!(".");
            //         } else {
            //             print!(" ");
            //         }
            //     }
            //     println!()
            // }

            break;
        }
    }

    step
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/14.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(12, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(1, res)
    }
}
