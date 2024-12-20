use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/07.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 7");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Input {
    expressions: Vec<Expression>,
}

#[derive(Clone)]
struct Expression {
    expected: i64,
    values: Vec<i64>,
    sizes: Vec<usize>,
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let expressions = input.lines().map(|l| {
            let parts = l.splitn(2, ": ").collect::<Vec<&str>>();
            let expected = i64::from_str(parts[0]).expect("cannot parse expected value");
            let strings = parts[1].split(" ").collect::<Vec<&str>>();
            let values = strings.iter().map(|v| {
                i64::from_str(v).expect("cannot parse value")
            }).collect::<Vec<i64>>();
            let sizes = strings.iter().map(|v| v.len()).collect();
            Expression {
                expected,
                values,
                sizes,
            }
        }).collect::<Vec<Expression>>();
        Ok(Input {
            expressions
        })
    }
}


fn part1(input: Input) -> i64 {
    input.expressions.iter().fold(0, |acc, expr: &Expression| {
        acc + part1_queue_reversed(expr.expected, &expr.values)
    })
}

fn part1_queue_reversed(expected: i64, values: &[i64]) -> i64 {
    let mut queue: Vec<Pos> = Vec::new();
    let last = values.len();
    let start = Pos::new(expected, last);
    queue.push(start);
    while let Some(element) = queue.pop() {
        if element.idx > 0 {
            let next_idx = element.idx - 1;
            let next = values[next_idx];

            // can be divided?
            if let Some(pos) = element.div(next) {
                if pos.value == 1 && next_idx == 0 {
                    return expected;
                } else {
                    queue.push(pos);
                }
            }

            // can subtract?
            if let Some(pos) = element.sub(next) {
                if pos.value > 0 {
                    queue.push(pos);
                } else if pos.value == 1 && next_idx == 0 {
                    return expected;
                }
            }
        }
    }
    0
}


#[derive(Eq, PartialEq)]
struct Pos {
    value: i64,
    idx: usize,
}
impl Pos {
    fn new(value: i64, idx: usize) -> Self {
        Pos { value, idx }
    }

    fn div(&self, other: i64) -> Option<Self> {
        if self.value % other == 0 {
            Some(
                Pos {
                    value: self.value / other,
                    idx: self.idx - 1,
                }
            )
        } else {
            None
        }
    }

    fn sub(&self, other: i64) -> Option<Self> {
        Some(
            Pos {
                value: self.value - other,
                idx: self.idx - 1,
            }
        )
    }

    fn drop(&self, other: i64, size: usize) -> Option<Self> {
        let scale = i64::pow(10, size as u32);

        if self.value % scale == other {
            let val = (self.value - other) / scale;
            Some(Pos {
                value: val,
                idx: self.idx - 1,
            })
        } else {
            None
        }
    }
}

fn part2(input: Input) -> i64 {
    input.expressions.iter().fold(0, |acc, expr: &Expression| {
        acc + part2_queue_reversed(expr.expected, &expr.values, &expr.sizes)
    })
}


fn part2_queue_reversed(expected: i64, values: &[i64], sizes: &[usize]) -> i64 {
    let mut queue: Vec<Pos> = Vec::new();
    let last = values.len();
    let start = Pos::new(expected, last);
    queue.push(start);
    while let Some(element) = queue.pop() {
        if element.idx > 0 {
            let next_idx = element.idx - 1;
            let size = sizes[next_idx];
            let next = values[next_idx];

            // can be divided?
            if let Some(pos) = element.div(next) {
                if pos.value == 1 && next_idx == 0 {
                    return expected;
                } else {
                    queue.push(pos);
                }
            }

            //can drop end?
            if next == element.value {
                //Concat will return empty string
                if next_idx == 0 {
                    // we are done
                    return expected;
                }
            } else if let Some(pos) = element.drop(next, size) {
                if pos.value > 0 {
                    queue.push(pos);
                } else if pos.value == 0 && next_idx == 0 {
                    return expected;
                }
            }

            // can subtract?
            if let Some(pos) = element.sub(next) {
                if pos.value > 0 {
                    queue.push(pos);
                } else if pos.value == 1 && next_idx == 0 {
                    return expected;
                }
            }
        }
    }
    0
}


// impl Pos {
//     fn new(value: i64, idx: usize) -> Self {
//         Pos { value, idx }
//     }
//
//     fn mul(&self, other: i64) -> Self {
//         Pos {
//             value: self.value * other,
//             idx: self.idx + 1,
//         }
//     }
//
//     fn sum(&self, other: i64) -> Self {
//         Pos {
//             value: self.value + other,
//             idx: self.idx + 1,
//         }
//     }
//
//     fn concat(&self, other: i64, size: usize) -> Self {
//         let val = self.value * (i64::pow(10, size as u32)) + other;
//         Pos {
//             value: val,
//             idx: self.idx + 1,
//         }
//     }
// }


// fn part1_queue_stack(expected: i64, values: &[i64]) -> i64 {
//     let mut queue: VecDeque<Pos> = VecDeque::new();
//     let start = Pos::new(values[0], 0);
//     queue.push_back(start);
//     while let Some(element) = queue.pop_back() {
//         if element.value > expected {
//             continue;
//         } else if element.value == expected && element.idx == values.len() - 1 {
//             return expected;
//         } else if element.idx < values.len() - 1 {
//             let next = values[element.idx + 1];
//             let mul_pos = element.mul(next);
//             if mul_pos.value <= expected {
//                 queue.push_back(mul_pos);
//             }
//             let sum_pos = element.sum(next);
//             if sum_pos.value <= expected {
//                 queue.push_back(sum_pos);
//             }
//         }
//     }
//     0
// }

// fn part2_queue_stack(expected: i64, values: &[i64], sizes: &[usize]) -> i64 {
//     let mut queue: Vec<Pos> = Vec::new();
//     let start = Pos::new(values[0], 0);
//     queue.push(start);
//     while let Some(element) = queue.pop() {
//         if element.value > expected {
//             continue;
//         } else if element.value == expected && element.idx == sizes.len() - 1 {
//             return expected;
//         } else if element.idx < sizes.len() - 1 {
//             let size = sizes[element.idx + 1];
//             let next = values[element.idx + 1];
//             let mul_pos = element.mul(next);
//             if mul_pos.value <= expected {
//                 queue.push(mul_pos);
//             }
//             let sum_pos = element.sum(next);
//             if sum_pos.value <= expected {
//                 queue.push(sum_pos);
//             }
//             let concat_pos = element.concat(next, size);
//             if concat_pos.value <= expected {
//                 queue.push(concat_pos);
//             }
//         }
//     }
//     0
// }


// fn part1_slow(input: Input) -> i64 {
//     input.expressions.iter().fold(0, |acc, expr: &Expression| {
//         let val = part1_rec(0, 0, Operand::Mul, expr.expected, &expr.values);
//         if val > 0 {
//             acc + expr.expected
//         } else {
//             acc
//         }
//     })
// }
//
//
// #[derive(Eq, PartialEq)]
// enum Operand {
//     Plus,
//     Mul,
// }
//
// fn part1_rec(i: usize, value: i64, expr: Operand, expected: i64, values: &Vec<String>) -> i64 {
//     if i == values.len() {
//         if value == expected {
//             expected
//         } else {
//             0
//         }
//     } else {
//         let current = i64::from_str(values[i].as_str()).expect("cannot parse value");
//         let value =
//             if i == 0 {
//                 current
//             } else {
//                 match expr {
//                     Operand::Plus => value + current,
//                     Operand::Mul => value * current,
//                 }
//             };
//         part1_rec(i + 1, value, Operand::Mul, expected, values)
//             +
//             part1_rec(i + 1, value, Operand::Plus, expected, values)
//     }
// }
//
//
// fn part2_slow(input: Input) -> i64 {
//     input.expressions.iter().fold(0, |acc, expr: &Expression| {
//         // if expr.expected != 7290 {
//         //     return acc;
//         // }
//         let val = part2_rec(0, "".to_string(), OperandMII::Concat, expr.expected, &expr.values);
//         // println!("Expected: {}, values: {:?}, Result: {}", expr.expected, expr.values, val);
//         if val {
//             acc + expr.expected
//         } else {
//             acc
//         }
//     })
// }
//
// #[derive(Eq, PartialEq, Debug)]
// enum OperandMII {
//     Plus,
//     Mul,
//     Concat,
// }
// fn part2_rec(i: usize, value: String, expr: OperandMII, expected: i64, values: &Vec<String>) -> bool {
//     if i == values.len() {
//         let val = i64::from_str(value.as_str()).expect("cannot parse value");
//         val == expected
//     } else {
//         let new_value = if i == 0 {
//             values[i].clone()
//         } else {
//             let current = i64::from_str(value.as_str()).expect("cannot parse value");
//             let new = i64::from_str(values[i].as_str()).expect("cannot parse value");
//             if current > expected {
//                 return false;
//             }
//             match expr {
//                 OperandMII::Plus =>
//                     (current + new).to_string(),
//                 OperandMII::Mul =>
//                     (current * new).to_string(),
//                 OperandMII::Concat => {
//                     let mut tmp = value.clone();
//                     tmp.push_str(values[i].as_str());
//                     tmp
//                 }
//             }
//         };
//         // println!("Expr {:?}, value: {:?}, curr: {}", expr, value.clone(), values[i]);
//         let result = part2_rec(i + 1, new_value.clone(), OperandMII::Mul, expected, values)
//             ||
//             part2_rec(i + 1, new_value.clone(), OperandMII::Plus, expected, values)
//             ||
//             part2_rec(i + 1, new_value, OperandMII::Concat, expected, values);
//         result
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/07.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(3749, res)
    }

    #[test]
    fn test_part1_stack() {
        let s = fs::read_to_string("data/07.txt").unwrap();
        let input = Input::from_str(s.as_str()).expect("cannot parse input");
        let _res = part1(input);
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(11387, res)
    }
    #[test]
    fn test_part2_stack() {
        let s = fs::read_to_string("data/07.txt").unwrap();
        let input = Input::from_str(s.as_str()).expect("cannot parse input");
        let _res = part2(input);
    }
}
