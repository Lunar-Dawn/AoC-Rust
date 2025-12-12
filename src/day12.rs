use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::util;

struct Present {
    shape: Vec<Vec<bool>>,
    size: u64,
    rect_size: u64,
}
impl Present {
    fn new<I>(lines: &mut I) -> Present
    where
        I: Iterator<Item = String>,
    {
        let shape: Vec<_> = lines
            .take_while(|s| !s.is_empty())
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        let size = shape
            .iter()
            .map(|l: &Vec<bool>| l.iter().map(|x| *x as u64).sum::<u64>())
            .sum::<u64>();
        let rect_size = shape.len() as u64 * shape[0].len() as u64;
        Present {
            shape,
            size,
            rect_size,
        }
    }
}

fn can_fit_presents(problem: &String, presents: &Vec<Present>) -> bool {
    let mut split = problem.split_whitespace();
    let size_str = split.next().unwrap();
    let size_split = size_str.find('x').unwrap();

    let width = size_str[0..size_split].parse::<u64>().unwrap();
    let height = size_str[size_split + 1..(size_str.len() - 1)]
        .parse::<u64>()
        .unwrap();
    let area = width * height;

    let present_counts: Vec<_> = split
        .enumerate()
        .map(|(i, s)| (&presents[i], s.parse::<u64>().unwrap()))
        .collect();

    let total_squares_needed = present_counts.iter().map(|(p, n)| p.size * n).sum::<u64>();
    if area < total_squares_needed {
        return false;
    }

    let total_rectangle_area = present_counts
        .iter()
        .map(|(p, n)| p.rect_size * n)
        .sum::<u64>();
    if total_rectangle_area <= area {
        return true;
    }

    unreachable!("Shouldn't be true")
}

pub fn part1(path: &PathBuf) -> util::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let mut presents = Vec::new();
    let mut total = 0;

    loop {
        let line = match lines.next() {
            None => break,
            Some(l) => l,
        };

        if !line.contains('x') {
            presents.push(Present::new(&mut lines));
        } else {
            if can_fit_presents(&line, &presents) {
                total += 1;
            }
        }
    }

    Ok(total.to_string())
}

pub fn part2(_: &PathBuf) -> util::Result<String> {
    Ok("There is no part 2".to_string())
}
