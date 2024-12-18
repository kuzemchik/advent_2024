use regex::Regex;
use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/13.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 13");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}


#[derive(Clone)]
struct Input {
    quizzes: Vec<Quiz>,
}

#[derive(Debug)]
enum InputError {}


#[derive(Clone)]
struct Pos {
    x: i64,
    y: i64,
}


impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Clone)]
struct Quiz {
    a: Pos,
    b: Pos,
    target: Pos,
}

fn parse_button(string: &str, re: &Regex) -> (i64, i64) {
    let (x, y) = re.captures(string).map(|c| {
        let x_sign = c.name("x_sign").expect("cannot parse sign");
        let x_string = c.name("x").expect("cannot parse sign");
        let y_sign = c.name("y_sign").expect("cannot parse sign");
        let y_string = c.name("y").expect("cannot parse sign");
        let mut x = i64::from_str(x_string.as_str()).expect("cannot parse x");
        if x_sign.as_str() == "-" {
            x *= -1;
        }
        let mut y = i64::from_str(y_string.as_str()).expect("cannot parse x");
        if y_sign.as_str() == "-" {
            y *= -1;
        }
        (x, y)
    }).unwrap();
    (x, y)
}

fn parse_target(string: &str, re: &Regex) -> (i64, i64) {
    let (x, y) = re.captures(string).map(|c| {
        let x_string = c.name("x").expect("cannot parse sign");

        let y_string = c.name("y").expect("cannot parse sign");
        let x = i64::from_str(x_string.as_str()).expect("cannot parse x");

        let y = i64::from_str(y_string.as_str()).expect("cannot parse x");

        (x, y)
    }).unwrap();
    (x, y)
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let button_re: Regex =
            Regex::new("Button [AB]: X(?<x_sign>[-+])(?<x>\\d+), Y(?<y_sign>[-+])(?<y>\\d+)")
                .unwrap();
        let target_re: Regex = Regex::new("Prize: X=(?<x>\\d+), Y=(?<y>\\d+)").unwrap();
        let quizes = input.split("\n\n").map(|ch| {
            let rows: Vec<&str> = ch.lines().collect();
            let a = parse_button(rows[0], &button_re);
            let b = parse_button(rows[1], &button_re);
            let target = parse_target(rows[2], &target_re);
            Quiz {
                a: Pos::new(a.0, a.1),
                b: Pos::new(b.0, b.1),
                target: Pos::new(target.0, target.1),
            }
        }).collect();
        Ok(Input {
            quizzes: quizes
        })
    }
}

fn part1(input: Input) -> i64 {
    let scale: i64 = 0;
    calculate(input, scale)
}

fn part2(input: Input) -> i64 {
    let scale: i64 = 10000000000000;
    calculate(input, scale)
}

fn calculate(input: Input, scale: i64) -> i64 {
    let mut result = 0;
    for quiz in input.quizzes {
        let mut a = -1;
        {
            let top = (quiz.target.x + scale) * quiz.b.y - (quiz.target.y + scale) * quiz.b.x;
            let bottom = quiz.a.x * quiz.b.y - quiz.a.y * quiz.b.x;
            if top % bottom == 0 {
                a = top / bottom;
            }
        }
        let mut b = -1;
        {
            let top = (quiz.target.y + scale) * quiz.a.x - (quiz.target.x + scale) * quiz.a.y;
            let bottom = quiz.a.x * quiz.b.y - quiz.a.y * quiz.b.x;
            if top % bottom == 0 {
                b = top / bottom;
            }
        }
        if a != -1 && b != -1 {
            result += a * 3 + b;
        }
    }
    result
}


// #[derive(Clone, Eq, PartialEq)]
// struct State {
//     num_presses: i64,
//     cost: i64,
//     x: i64,
//     y: i64,
// }
//
// impl Ord for State {
//     fn cmp(&self, other: &Self) -> Ordering {
//         (self.x + self.y).cmp(&(other.x + other.y))
//             .then(self.cost.cmp(&other.cost))
//     }
// }
//
// impl PartialOrd for State {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(&other))
//     }
// }
//
// impl State {
//     fn press(&self, pos: &Pos, cost: i64) -> Option<State> {
//         let state = State {
//             num_presses: self.num_presses + 1,
//             cost: self.cost + cost,
//             x: self.x - pos.x,
//             y: self.y - pos.y,
//
//         };
//         if state.x < 0 || state.y < 0 {
//             None
//         } else {
//             Some(state)
//         }
//     }
// }
//
// fn part1(input: Input) -> i64 {
//     let mut result = 0;
//     for quiz in input.quizes {
//         let mut queue = BinaryHeap::new();
//         let start = State {
//             num_presses: 0,
//             cost: 0,
//             x: quiz.target.x,
//             y: quiz.target.y,
//         };
//         queue.push(Reverse(start.clone()));
//         let mut min_cost = i64::MAX;
//         let mut states = HashMap::new();
//         let mut num_created = 0;
//         let mut num_processed = 0;
//         let mut num_visited = 0;
//         let mut num_updated = 0;
//         let mut num_missed = 0;
//         while let Some(Reverse(state)) = queue.pop() {
//             if state.x == 0 && state.y == 0 {
//                 num_processed += 1;
//                 min_cost = min_cost.min(state.cost);
//             } else if state.x < 0 || state.y < 0 {
//                 num_missed += 1;
//                 continue;
//             }
//             if let Some(prev_cost) = states.get_mut(&(state.x, state.y)) {
//                 if *prev_cost <= state.cost {
//                     num_visited += 1;
//                     continue;
//                 } else {
//                     num_updated += 1;
//                     *prev_cost = state.cost;
//                 }
//             } else {
//                 states.insert((state.x, state.y), state.cost);
//             }
//
//             if let Some(press_a) = state.press(&quiz.a, 3) {
//                 if let Some(prev_cost) = states.get_mut(&(press_a.x, press_a.y)) {
//                     if *prev_cost > state.cost {
//                         queue.push(Reverse(press_a));
//                     } else {
//                         num_processed += 1;
//                     }
//                 } else {
//                     queue.push(Reverse(press_a))
//                 }
//             }
//             if let Some(press_b) = state.press(&quiz.b, 1) {
//                 if let Some(prev_cost) = states.get_mut(&(press_b.x, press_b.y)) {
//                     if *prev_cost > state.cost {
//                         queue.push(Reverse(press_b))
//                     } else {
//                         num_processed += 1;
//                     }
//                 } else {
//                     queue.push(Reverse(press_b))
//                 }
//             }
//             num_created += 2;
//             if num_created % 10000000 == 0 {
//                 println!("cr:{}, pr:{}, ms:{}, vs: {}, ud {}", num_created, num_processed, num_missed, num_visited, num_updated)
//             }
//         }
//         if min_cost != i64::MAX {
//             result += min_cost;
//         }
//     }
//     result
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/13.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(480, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(875318608908, res)
    }
}
