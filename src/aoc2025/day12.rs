use crate::dyn_result::DynResult;
use crate::runner;

runner!();

struct Tree {
    width: u64,
    height: u64,
    present_counts: Vec<u64>,
}
impl Tree {
    fn parse(line: &str) -> DynResult<Tree> {
        let mut split = line.split_whitespace();
        let Some(size_str) = split.next() else {
            return Err(format!("Failed to find dimension on line {line}").into());
        };
        let Some(size_split) = size_str.find('x') else {
            return Err(format!("Failed to parse dimensions: {size_str}").into());
        };

        let width = size_str[0..size_split].parse::<u64>()?;
        let height = size_str[size_split + 1..(size_str.len() - 1)].parse::<u64>()?;

        let present_counts: Vec<_> = split.map(|s| s.parse::<u64>()).collect::<Result<_, _>>()?;

        Ok(Tree {
            width,
            height,
            present_counts,
        })
    }

    fn can_fit_presents(&self, presents: &Vec<Present>) -> bool {
        let area = self.width * self.height;

        let (total_squares_needed, total_rectangle_area) = self
            .present_counts
            .iter()
            .enumerate()
            .map(|(i, n)| (&presents[i], n))
            .map(|(p, n)| (p.size * n, p.rect_size * n))
            .fold((0, 0), |l, r| (l.0 + r.0, l.1 + r.1));

        if area < total_squares_needed {
            return false;
        }

        if total_rectangle_area <= area {
            return true;
        }

        unreachable!("Shouldn't be true")
    }
}
struct Input {
    presents: Vec<Present>,
    trees: Vec<Tree>,
}
fn parse(input: &str) -> DynResult<Input> {
    let mut lines = input.lines().peekable();

    let mut presents = Vec::new();

    loop {
        let Some(line) = lines.peek() else {
            return Err("No problems appeared in data set".into());
        };

        if line.contains('x') {
            break;
        }

        presents.push(Present::new(&mut lines));
    }

    let trees = lines.map(Tree::parse).collect::<Result<_, _>>()?;

    Ok(Input { presents, trees })
}

struct Present {
    size: u64,
    rect_size: u64,
}
impl Present {
    fn new<'a, I>(lines: &mut I) -> Present
    where
        I: Iterator<Item = &'a str>,
    {
        let shape: Vec<Vec<bool>> = lines
            .take_while(|s| !s.is_empty())
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();

        let rect_size = shape.len() as u64 * shape[0].len() as u64;

        let size = shape
            .into_iter()
            .map(|l| l.iter().map(|x| *x as u64).sum::<u64>())
            .sum::<u64>();
        Present { size, rect_size }
    }
}

fn part1(input: &Input) -> usize {
    input
        .trees
        .iter()
        .filter(|t| t.can_fit_presents(&input.presents))
        .count()
}

fn part2(_: &Input) -> &'static str {
    "There is no part 2"
}
