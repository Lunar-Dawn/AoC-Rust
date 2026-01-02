use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::point2i::Point2i;
use crate::util::vec2i::Vec2i;
use crate::util::DynResult;

runner!();

type ParsedData = VectorGrid<char>;
fn parse(input: &str) -> DynResult<ParsedData> {
    let lines: Vec<_> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut data = Vec::with_capacity(width * height);
    for line in lines {
        data.extend(line.chars());
    }

    Ok(VectorGrid::from(width, height, data))
}

fn test_dir(grid: &ParsedData, pos: &Point2i, dir: &Vec2i) -> bool {
    let mut pos = pos.clone();
    if !matches!(grid.get(&pos), Some('X')) {
        return false;
    }
    pos += dir;
    if !matches!(grid.get(&pos), Some('M')) {
        return false;
    }
    pos += dir;
    if !matches!(grid.get(&pos), Some('A')) {
        return false;
    }
    pos += dir;
    matches!(grid.get(&pos), Some('S'))
}
fn test_pos(grid: &ParsedData, pos: &Point2i) -> usize {
    Vec2i::DIRECTIONS
        .iter()
        .filter(|d| test_dir(grid, pos, d))
        .count()
}
fn part1(grid: &ParsedData) -> usize {
    grid.pos_iter().map(|p| test_pos(grid, &p)).sum()
}

fn test_mas(grid: &ParsedData, pos: &Point2i, dir: &Vec2i) -> bool {
    let forward = match grid.get(&(pos + dir)) {
        Some('M') => 'M',
        Some('S') => 'S',
        _ => return false,
    };
    let backwards = match grid.get(&(pos - dir)) {
        Some('M') => 'M',
        Some('S') => 'S',
        _ => return false,
    };
    forward != backwards
}
fn test_cross_pos(grid: &ParsedData, pos: &Point2i) -> bool {
    if !matches!(grid.get(&pos), Some('A')) {
        return false;
    }

    test_mas(grid, pos, &Vec2i::UP_LEFT) && test_mas(grid, pos, &Vec2i::UP_RIGHT)
}
fn part2(grid: &ParsedData) -> usize {
    grid.pos_iter().filter(|p| test_cross_pos(grid, p)).count()
}
