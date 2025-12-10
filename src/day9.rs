use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul, Neg, Sub};
use std::path::PathBuf;

use crate::util;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    const fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
    fn area(&self, other: &Point) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
    fn signum(&self) -> Point {
        Point {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
    fn magnitude(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl Mul<i64> for Point {
    type Output = Self;
    fn mul(self, factor: i64) -> Self::Output {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

fn parse_points(path: &PathBuf) -> Vec<Point> {
    fn parse_point(string: &str) -> Point {
        let offsets: Vec<_> = string.split(',').map(|s| s.parse().unwrap()).collect();

        Point::new(offsets[0], offsets[1])
    }

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    lines
        .map(|l| l.unwrap())
        .map(|l| parse_point(l.as_str()))
        .collect()
}

pub fn part1(path: &PathBuf) -> util::Result<String> {
    let points = parse_points(path);

    let mut max_area = 0;
    for p1 in &points {
        for p2 in &points {
            let area = p1.area(p2);
            max_area = max(max_area, area);
        }
    }

    Ok(max_area.to_string())
}

// TODO: Check line intersections instead
fn test_rectangle(p1: &Point, p2: &Point, outline: &HashSet<Point>) -> Option<i64> {
    let min_x = min(p1.x, p2.x);
    let max_x = max(p1.x, p2.x);
    let min_y = min(p1.y, p2.y);
    let max_y = max(p1.y, p2.y);

    for p in outline {
        if min_x < p.x && p.x < max_x && min_y < p.y && p.y < max_y {
            return None;
        }
    }

    Some(p1.area(p2))
}

pub fn part2(path: &PathBuf) -> util::Result<String> {
    let points = parse_points(path);

    let mut outline = HashSet::new();
    for i in 0..points.len() {
        let from = points[i];
        let to = points[(i + 1) % points.len()];

        let difference = to - from;
        let dir = difference.signum();

        for i in 0..difference.magnitude() {
            outline.insert(from + dir * i);
        }
    }

    let mut max_area = 0;
    for point_1 in 0..points.len() {
        for point_2 in (point_1 + 1)..points.len() {
            match test_rectangle(&points[point_1], &points[point_2], &outline) {
                None => continue,
                Some(area) => max_area = max(max_area, area),
            }
        }
    }

    Ok(max_area.to_string())
}
