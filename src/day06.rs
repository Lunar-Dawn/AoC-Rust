use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::util;

pub fn part1(path: &PathBuf) -> util::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let rows: Vec<_> = lines.map(|l| l.unwrap()).collect();

    let numbers: Vec<_> = rows
        .iter()
        .take(rows.len() - 1)
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let operations: Vec<_> = rows
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| String::from(s))
        .collect();

    let mut total = 0;
    for i in 0..operations.len() {
        let operation = &operations[i];
        let column = numbers.iter().map(|r| &r[i]);
        total += match operation.as_str() {
            "+" => column.sum::<u64>(),
            "*" => column.product::<u64>(),
            _ => panic!("unknown operation {}", operation),
        }
    }

    Ok(total.to_string())
}

pub fn part2(path: &PathBuf) -> util::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut rows: Vec<_> = lines.map(|l| l.unwrap()).collect();
    let operation_row = rows.pop().unwrap();
    let mut current_operation_start_pos = 0;

    let mut total = 0;
    while current_operation_start_pos < operation_row.len() {
        let operator = operation_row[current_operation_start_pos..]
            .chars()
            .next()
            .unwrap();
        let column_width =
            match operation_row[current_operation_start_pos + 1..].find(|c| c == '+' || c == '*') {
                Some(column_width) => column_width,
                None => operation_row.len() - current_operation_start_pos,
            };

        let numbers = rows
            .iter()
            .map(|l| &l[current_operation_start_pos..current_operation_start_pos + column_width])
            .collect::<Vec<_>>();
        current_operation_start_pos += column_width + 1;

        let (mut result, operation): (u64, fn(u64, u64) -> u64) = match operator {
            '+' => (0, |l, r| l + r),
            '*' => (1, |l, r| l * r),
            _ => panic!("unknown operator {}", operator),
        };

        for i in 0..column_width {
            let num = numbers
                .iter()
                .map(|s| s.chars().nth(i).unwrap())
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u64>()?;
            result = operation(result, num);
        }
        total += result;
    }

    Ok(total.to_string())
}
