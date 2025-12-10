use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::util;

pub fn part1(path: &PathBuf) -> util::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut position = 50;

    for line in reader.lines() {
        let line = line?;

        let direction = match &line.chars().nth(0) {
            Some('L') => -1,
            _ => 1,
        };
        let steps = direction * str::parse::<i32>(&line[1..])?;

        position = (position + steps).rem_euclid(100);
        if position == 0 {
            total += 1;
        }
    }

    Ok(total.to_string())
}

#[allow(unused_variables)]
pub fn part2(path: &PathBuf) -> util::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut position = 50;

    for line in reader.lines() {
        let line = line?;

        let direction: i32 = match &line.chars().nth(0) {
            Some('L') => -1,
            _ => 1,
        };
        let steps = str::parse::<i32>(&line[1..])?;

        for _ in 0..steps {
            position = (position + direction).rem_euclid(100);

            if position == 0 {
                total += 1;
            }
        }
    }

    Ok(total.to_string())
}
