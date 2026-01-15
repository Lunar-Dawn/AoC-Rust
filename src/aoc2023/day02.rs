use crate::runner;
use crate::util::parse::take_integers;
use crate::util::DynResult;

runner!();

type Round = (u8, u8, u8);
struct Game {
    id: u8,
    rounds: Vec<Round>,
}
impl Game {
    fn new(str: &str) -> DynResult<Self> {
        let (str, [id]) = take_integers(str)?;
        let str = &str[2..];

        let mut rounds = Vec::new();

        for round_str in str.split(';') {
            rounds.push(Self::parse_round(round_str)?);
        }

        Ok(Self { id, rounds })
    }
    fn parse_round(str: &str) -> DynResult<Round> {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for cube_str in str.split(',') {
            let (cube_str, [n]) = take_integers(cube_str)?;
            let cube_str = &cube_str[1..];

            match cube_str {
                "red" => r = n,
                "green" => g = n,
                "blue" => b = n,
                _ => unreachable!(),
            }
        }

        Ok((r, g, b))
    }

    fn playable_with(&self, r_max: u8, g_max: u8, b_max: u8) -> bool {
        !self
            .rounds
            .iter()
            .any(|&(r, g, b)| r > r_max || g > g_max || b > b_max)
    }
    fn cubes_needed(&self) -> (u8, u8, u8) {
        self.rounds
            .iter()
            .fold((0, 0, 0), |(r1, g1, b1), &(r2, g2, b2)| {
                (r1.max(r2), g1.max(g2), b1.max(b2))
            })
    }
}

type ParsedData = Vec<Game>;
fn parse(input: &str) -> DynResult<ParsedData> {
    Ok(input.lines().map(Game::new).collect::<Result<_, _>>()?)
}

fn part1(games: &ParsedData) -> u64 {
    games
        .iter()
        .filter(|g| g.playable_with(12, 13, 14))
        .map(|g| g.id as u64)
        .sum()
}
fn part2(games: &ParsedData) -> u64 {
    games
        .iter()
        .map(|game| game.cubes_needed())
        .map(|(r, g, b)| r as u64 * g as u64 * b as u64)
        .sum::<u64>()
}
