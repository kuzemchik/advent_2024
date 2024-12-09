use std::fs;
use std::str::FromStr;

pub fn print() {
    let s = fs::read_to_string("data/09.txt").unwrap();
    let input = Input::from_str(s.as_str()).expect("cannot parse input");

    println!("Day 9");
    println!("Part 1 {}", part1(input.clone()));
    println!("Part 2 {}", part2(input.clone()));
}

#[derive(Clone)]
struct Input {
    data: Vec<Chunk>,
}

#[derive(Debug)]
enum InputError {}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data: Vec<u8> = input.trim().chars().map(|c|
            u8::from_str(String::from(c).as_str()).expect(format!("cannot parse number: {}", c).as_str())
        ).collect();

        let parsed = data.chunks(2).enumerate().flat_map(|(i, v)| {
            let mut res = vec![];
            res.push(Chunk {
                id: i as i64,
                len: v[0],
            });
            if v.len() > 1 && v[1] > 0 {
                res.push(Chunk {
                    id: -1,
                    len: v[1],
                });
            }
            res
        }).collect::<Vec<Chunk>>();
        Ok(Input {
            data: parsed
        })
    }
}


#[derive(Clone)]
struct Chunk {
    id: i64,
    len: u8,
}

fn part1(input: Input) -> i64 {
    let mut parsed = input.data.clone();
    let mut start = 0;
    let mut end = parsed.len() - 1;

    let mut checksum_value: i64 = 0;
    let mut checksum_idx = 0;
    // let mut string = vec![];
    loop {
        if start > end {
            break;
        }
        let current_chunk = &mut parsed[start];
        if current_chunk.id != -1 {
            for _ in 0..current_chunk.len {
                checksum_value += checksum_idx * current_chunk.id;
                // string.push(current_chunk.id);
                checksum_idx += 1;
            }
        } else {
            let mut available = current_chunk.len;
            while start < end {
                if parsed[end].id != -1 {
                    let last = &mut parsed[end];
                    while last.len != 0 && available != 0 {
                        checksum_value += checksum_idx * last.id;
                        // string.push(last.id);
                        checksum_idx += 1;
                        available -= 1;
                        last.len -= 1;
                    }
                }
                if available == 0 {
                    break;
                }
                end -= 1
            }
        }
        start += 1
    }
    // println!("compacted: {:?}", &string);
    // println!("original_checksum: {}", checksum(&string));

    checksum_value
}


fn part2(input: Input) -> i64 {
    let mut parsed = input.data.clone();
    let mut start = 0;
    let mut max_end = parsed.len() - 1;

    let mut checksum_value: i64 = 0;
    let mut checksum_idx = 0;
    // let mut string = vec![];
    loop {
        if start >= max_end {
            break;
        }
        let current_chunk = &mut parsed[start];
        if current_chunk.id != -1 {
            for _ in 0..current_chunk.len {
                checksum_value += checksum_idx * current_chunk.id;
                // string.push(current_chunk.id);
                checksum_idx += 1;
            }
        } else {
            let mut available = current_chunk.len;
            let mut end = max_end;
            while start < end && parsed[end].id == -1 {
                end -= 1
            }
            max_end = max_end.min(end + 1);

            while start < end {
                if start >= end {
                    break;
                } else if parsed[end].id != -1 && parsed[end].len <= available {
                    let last = &mut parsed[end];
                    for _ in 0..last.len {
                        checksum_value += checksum_idx * last.id;
                        // string.push(last.id);
                        checksum_idx += 1;
                        available -= 1;
                    }
                    last.id = -1;
                }
                if available == 0 {
                    break;
                }
                end -= 1;
            }
            for _ in 0..available {
                checksum_idx += 1;
                // string.push(-1);
            }
        }
        start += 1
    }
    // println!("compacted: {:?}", &string);
    // println!("original_checksum: {}", checksum(&string));

    checksum_value
}


// fn part1(input: Input) -> i64 {
//     let mut unpacked = input.data.chunks(2).enumerate().flat_map(|(i, v)| {
//         let mut res = vec![];
//         res.append(&mut vec![i as i64; v[0] as usize]);
//         let empty_len = if v.len() > 1 {
//             v[1]
//         } else {
//             0
//         };
//         res.append(&mut vec![-1; empty_len as usize]);
//         res
//     }).collect::<Vec<i64>>();
//     // println!("unpacked: {:?}", &unpacked);
//     let mut start = 0;
//     let mut end = unpacked.len() - 1;
//     loop {
//         while start <= end && unpacked[start] != -1 {
//             start += 1;
//         }
//         while end >= start && unpacked[end] == -1 {
//             end -= 1;
//         }
//         if start >= end {
//             break;
//         }
//         unpacked[start] = unpacked[end];
//         unpacked[end] = -1;
//         end -= 1;
//     }
//     // println!("compressed: {:?}", &unpacked);
//
//     // println!("compressed: {:?}", &drop_empty);
//     let result = checksum(&unpacked);
//     result
// }


// fn part2(input: Input) -> i64 {
//     let mut unpacked = input.data.chunks(2).enumerate().flat_map(|(i, v)| {
//         let mut res = vec![];
//         res.append(&mut vec![i as i64; v[0] as usize]);
//         let empty_len = if v.len() > 1 {
//             v[1]
//         } else {
//             0
//         };
//         res.append(&mut vec![-1; empty_len as usize]);
//         res
//     }).collect::<Vec<i64>>();
//     // println!("unpacked: {:?}", &unpacked);
//     let mut start = 0;
//     loop {
//         let mut end = unpacked.len() - 1;
//         while start <= end && unpacked[start] != -1 {
//             start += 1;
//         }
//
//         while end >= start && unpacked[end] == -1 {
//             end -= 1;
//         }
//         if start >= end {
//             break;
//         }
//         let mut buff = vec![];
//         let file_id = unpacked[end];
//         let mut cached_start = start;
//         let mut cached_end = end;
//         while unpacked[cached_end] == file_id {
//             buff.push(file_id);
//             cached_end -= 1;
//             if unpacked[cached_start] == -1 {
//                 cached_start += 1;
//             }
//         }
//         if buff.len() == cached_start - start {
//             for v in buff {
//                 unpacked[start] = v;
//                 start += 1;
//                 end -= 1
//             }
//         }
//
//         unpacked[end] = -1;
//         end -= 1;
//     }
//     // println!("compressed: {:?}", &unpacked);
//     let drop_empty = unpacked.into_iter().take_while(|&v| v != -1).collect::<Vec<i64>>();
//     // println!("compressed: {:?}", &drop_empty);
//     let mut str = String::new();
//     drop_empty.iter().for_each(|v| {
//         str.push_str(v.to_string().as_str())
//     });
//     let result = checksum(&drop_empty);
//     result
// }

// fn checksum(data: &[i64]) -> i64 {
//     data.iter().enumerate().fold(0, |acc, (i, d)| {
//         if *d == -1 {
//             acc
//         } else {
//             acc + (i as i64 * *d)
//         }
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    fn input() -> Input {
        let s = fs::read_to_string("tests/09.txt").unwrap();
        Input::from_str(s.as_str()).expect("cannot parse input")
    }


    #[test]
    fn test_part1() {
        let res = part1(input());
        assert_eq!(1928, res)
    }

    #[test]
    fn test_part2() {
        let res = part2(input());
        assert_eq!(2858, res)
    }
}
