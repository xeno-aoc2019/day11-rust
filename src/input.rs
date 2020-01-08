use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Result, Lines};
use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive};


pub fn read_program<P>(file_name: P) -> Vec<BigInt>
    where P: AsRef<Path>, {
    if let Ok(lines) = get_lines(file_name) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                let mut result: Vec<BigInt> = vec!();
                for item in line.split(",") {
                    let byte: BigInt = item.parse::<BigInt>().unwrap();
                    result.push(byte);
                }
                return result;
            }
        }
    }
    panic!("no input");
}

fn get_lines<P>(file_name: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file).lines())
}