use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() {
    let nums = read(File::open("input").unwrap()).unwrap();
    part1(&nums);
    part2(&nums);
}

fn part1(nums: &Vec<i64>) {
    println!("\nPart 1:");
    for num1 in nums.iter() {
        for num2 in nums.iter() {
            if num1 + num2 == 2020 {
                println!("{} + {} == 2020, multiplied = {}", num1, num2, num1 * num2);
                return;
            }
        }
    }
}

fn part2(nums: &Vec<i64>) {
    println!("\nPart 2:");
    for num1 in nums.iter() {
        for num2 in nums.iter() {
            for num3 in nums.iter() {
                if num1 != num2 && num2 != num3 && num1 + num2 + num3 == 2020 {
                    println!(
                        "{} + {} + {} == 2020, multiplied = {}",
                        num1,
                        num2,
                        num3,
                        num1 * num2 * num3
                    );
                    return;
                }
            }
        }
    }
}
