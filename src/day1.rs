
use crate::files;

pub fn day1() {
    let instructions = instructions_from_file();

    let mut current_pos: i32 = 50;
    let mut zero_count = 0;
    let mut pass_zero_count = 0;

    for instruction in instructions {
        match instruction.direction {
            'L' => {
                for x in 0..instruction.amount {
                    current_pos -= 1;
                    current_pos = current_pos.rem_euclid(100);
                    let final_step = x == instruction.amount - 1;
                    if current_pos == 0 {
                        if final_step {
                            zero_count += 1;
                        } else {
                            pass_zero_count += 1;
                        }
                }
            }
        }
            'R' => {
                for x in 0..instruction.amount {
                    current_pos += 1;
                    current_pos = current_pos.rem_euclid(100);
                    let final_step = x == instruction.amount - 1;
                    if current_pos == 0 {
                        if final_step {
                            zero_count += 1;        // lands on zero
                        } else {
                            pass_zero_count += 1;   // passes through zero
                        }
                }
            }
        }
            _ => todo!(),
        }
    }

    println!("{}", zero_count + pass_zero_count);
}

pub fn instructions_from_file() -> Vec<Instructions> {
    let lines = files::read_lines("./inputday1.txt").unwrap();

    lines
        .map(|x| {
            let line = x.unwrap();
            Instructions {
                direction: line.chars().next().unwrap(),
                amount: line[1..].parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<Instructions>>()
}

pub struct Instructions {
    direction: char,
    amount: i32,
}