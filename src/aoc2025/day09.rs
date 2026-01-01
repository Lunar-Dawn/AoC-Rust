use std::cmp::{max, min};

use crate::dyn_result::DynResult;
use crate::runner;

runner!();

#[derive(Clone, Copy)]
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
}
fn parse_point(string: &str) -> DynResult<Point> {
    let offsets: Vec<_> = string
        .split(',')
        .map(|s| s.parse::<i64>())
        .collect::<Result<_, _>>()?;

    Ok(Point::new(offsets[0], offsets[1]))
}

struct Line {
    start: Point,
    end: Point,
}

struct Input {
    points: Vec<Point>,
    lines: Vec<Line>,
}
fn parse(input: &str) -> DynResult<Input> {
    let points: Vec<_> = input.lines().map(parse_point).collect::<Result<_, _>>()?;

    let mut lines = Vec::new();
    for i in 0..points.len() {
        lines.push(Line {
            start: points[i],
            end: points[(i + 1) % points.len()],
        })
    }

    Ok(Input { points, lines })
}

fn part1(input: &Input) -> i64 {
    let mut max_area = 0;
    for p1 in &input.points {
        for p2 in &input.points {
            let area = p1.area(p2);
            max_area = max(max_area, area);
        }
    }

    max_area
}
fn test_rectangle(p1: &Point, p2: &Point, lines: &Vec<Line>) -> Option<i64> {
    let min_x = min(p1.x, p2.x);
    let max_x = max(p1.x, p2.x);
    let min_y = min(p1.y, p2.y);
    let max_y = max(p1.y, p2.y);

    for line in lines {
        if line.start.x == line.end.x {
            let x = line.start.x;
            let l_min_y = min(line.start.y, line.end.y);
            let l_max_y = max(line.start.y, line.end.y);

            if min_x < x && x < max_x && l_min_y < max_y && l_max_y > min_y {
                return None;
            }
        } else {
            let y = line.start.y;
            let l_min_x = min(line.start.x, line.end.x);
            let l_max_x = max(line.start.x, line.end.x);

            if min_y < y && y < max_y && l_min_x < max_x && l_max_x > min_x {
                return None;
            }
        }
    }

    Some(p1.area(p2))
}

fn part2(input: &Input) -> i64 {
    let mut max_area = 0;
    for point_1 in 0..input.points.len() {
        for point_2 in (point_1 + 1)..input.points.len() {
            match test_rectangle(&input.points[point_1], &input.points[point_2], &input.lines) {
                None => continue,
                Some(area) => max_area = max(max_area, area),
            }
        }
    }

    max_area
}
