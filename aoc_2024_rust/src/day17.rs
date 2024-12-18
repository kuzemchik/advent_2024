use crate::day17::OpCode::*;
use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/17.txt").unwrap();
    let input = Executor::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 16");
    println!("Part 1 {}", part1(&mut input.clone()));
    println!("Part 2 {}", part2(&mut input.clone()));
}

#[derive(Clone, Debug)]
struct Executor {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
}

impl Executor {
    fn get_combo_op(&self, op_code: &OpCode) -> u64 {
        match op_code {
            Adv => 0,
            Bxl => 1,
            Bst => 2,
            Jnz => 3,
            Bxc => self.a,
            Out => self.b,
            Bdv => self.c,
            Cdv => unimplemented!("Shouldn't happen"),
        }
    }
    fn get_op(&self, op_code: &OpCode) -> u64 {
        match op_code {
            Adv => 0,
            Bxl => 1,
            Bst => 2,
            Jnz => 3,
            Bxc => 4,
            Out => 5,
            Bdv => 6,
            Cdv => 7,
        }
    }
    fn execute(&mut self) -> Vec<u8> {
        let mut idx = 0;
        let mut output: Vec<u8> = vec![];
        let program_codes: Vec<OpCode> = self.program.iter().map(|v| OpCode::new(*v)).collect();
        let mut op_code: &OpCode;
        let mut param: Option<&OpCode>;
        loop {
            if idx >= program_codes.len() {
                break;
            }
            op_code = &program_codes[idx];
            if idx + 1 < program_codes.len() {
                param = Some(&program_codes[idx + 1]);
            } else {
                param = None;
            }
            match (op_code, param) {
                (Adv, Some(op)) => {
                    let denom: u64 = u64::pow(2, self.get_combo_op(op) as u32);
                    self.a = self.a / denom;
                }
                (Bxl, Some(op)) => self.b = self.b ^ self.get_op(op),
                (Bst, Some(op)) => self.b = self.get_combo_op(op) % 8,
                (Jnz, Some(op)) => {
                    if self.a != 0 {
                        idx = self.get_op(op) as usize;
                        continue;

                        // do not jump
                    }
                }
                (Bxc, _) => self.b = self.b ^ self.c,
                (Out, Some(op)) => {
                    let value = (self.get_combo_op(op) % 8) as u8;
                    output.push(value);
                }
                (Bdv, Some(op)) => {
                    let denom: u64 = u64::pow(2, self.get_combo_op(op) as u32);
                    self.b = self.a / denom;
                }
                (Cdv, Some(op)) => {
                    let denom: u64 = u64::pow(2, self.get_combo_op(op) as u32);
                    self.c = self.a / denom;
                }
                _ => unimplemented!("unknown combination"),
            }
            // println!("{:?}", self);
            idx += 2
        }
        output
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    fn new(input: u8) -> OpCode {
        match input {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => panic!("CannotParseOptCode"),
        }
    }
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Executor {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let registers = parts[0]
            .lines()
            .map(|l| {
                u64::from_str(l.split(": ").last().expect("cannot extract string"))
                    .expect("cannot extract value")
            })
            .collect::<Vec<u64>>();
        let program = parts[1]
            .lines()
            .flat_map(|l| {
                l.trim().split_at(9).1.split(",").map(|v| {
                    // println!("Code '{}'", v);
                    u8::from_str(v).expect("cannot parse opcode")
                })
            })
            .collect::<Vec<u8>>();
        Ok(Executor {
            a: registers[0],
            b: registers[1],
            c: registers[2],
            program: program,
        })
    }
}

fn part1(input: &mut Executor) -> String {
    input
        .execute()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part2(executor: &mut Executor) -> u64 {
    for start in 0..u64::pow(2, 7) {
        if let Some(a) = match_recursive(0, start, executor) {
            return a;
        }
    }
    panic!("Not found")
}

fn match_recursive(idx: usize, lower: u64, executor: &mut Executor) -> Option<u64> {
    for upper in 0..8 {
        let a = upper << (3 * idx + 7) | lower;
        // println!("{:b} {:b}", upper, lower);
        // println!("{:b}", a);
        executor.a = a;
        let output = executor.execute();
        // println!("[{}]: {:?} {:?}", idx, output, executor.program);
        if output.len() <= executor.program.len()
            && output.len() > idx
            && output[idx] == executor.program[idx]
        {
            if output == executor.program {
                return Some(a);
            }
            if let Some(a) = match_recursive(idx + 1, a, executor) {
                return Some(a);
            }
        }
    }
    None
}

// slightly slower
// fn match_queue(idx: usize, start: u64, executor: &mut Executor) -> Option<u64> {
//     let mut stack = vec![];
//     stack.push((start, idx));
//     while let Some((lower, idx)) = stack.pop() {
//         for upper in (0..8).rev() {
//             let a = upper << (3 * idx + 7) | lower;
//             // println!("{:b} {:b}", upper, lower);
//             // println!("{:b}", a);
//             executor.a = a;
//
//             let output = executor.execute();
//             // println!("[{}]: {:?} {:?}", idx, output, executor.program);
//             if output.len() <= executor.program.len()
//                 && output.len() > idx
//                 && output[idx] == executor.program[idx]
//             {
//                 if output == executor.program {
//                     return Some(a);
//                 }
//                 stack.push((a, idx + 1))
//             }
//         }
//     }
//     None
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn input() -> Executor {
        let s = fs::read_to_string("tests/17.txt").unwrap();
        Executor::from_str(s.as_str()).expect("cannot parse input")
    }

    #[test]
    fn test_part1() {
        let res = part1(&mut input());
        assert_eq!("4,6,3,5,6,3,5,2,1,0", res)
    }

    #[test]
    fn test_part1_1() {
        let s = fs::read_to_string("tests/17_1.txt").unwrap();
        let mut input = Executor::from_str(s.as_str()).expect("cannot parse input");
        let _res = part1(&mut input);
        assert_eq!(1, input.b)
    }
    #[test]
    fn test_part1_2() {
        let s = fs::read_to_string("tests/17_2.txt").unwrap();
        let mut input = Executor::from_str(s.as_str()).expect("cannot parse input");
        let res = part1(&mut input);
        assert_eq!("0,1,2", res)
    }

    #[test]
    fn test_part1_3() {
        let s = fs::read_to_string("tests/17_3.txt").unwrap();
        let mut input = Executor::from_str(s.as_str()).expect("cannot parse input");
        let res = part1(&mut input);
        assert_eq!(0, input.a);
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", res)
    }

    #[test]
    fn test_part1_4() {
        let s = fs::read_to_string("tests/17_4.txt").unwrap();
        let mut input = Executor::from_str(s.as_str()).expect("cannot parse input");
        let _res = part1(&mut input);
        assert_eq!(26, input.b)
    }

    #[test]
    fn test_part1_5() {
        let s = fs::read_to_string("tests/17_5.txt").unwrap();
        let mut input = Executor::from_str(s.as_str()).expect("cannot parse input");
        let _res = part1(&mut input);
        assert_eq!(44354, input.b)
    }

    #[test]
    fn test_part2_2() {
        let s = fs::read_to_string("tests/17_6.txt").unwrap();
        let mut input = Executor::from_str(s.as_str()).expect("cannot parse input");
        let res = part2(&mut input);
        assert_eq!(117440, res)
    }
}
