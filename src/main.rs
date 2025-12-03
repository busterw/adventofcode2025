use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
    str::Chars,
};

fn main() {
    day1();
}

pub fn day1(){
    let instructions = instructions_from_file();

    let mut current_pos = 50;
    let mut zero_count = 0;

    for instruction in instructions {
        match instruction.direction {
            'L' => {
                current_pos -= instruction.amount;
            }
            'R' => current_pos += instruction.amount,
            _ => todo!(),
        }
        if current_pos % 100 == 0 {
            zero_count += 1
        }
    }

    println!("{}", zero_count)
}

pub fn instructions_from_file() -> Vec<Instructions> {
    let lines = read_lines("./input.txt").unwrap();

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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
