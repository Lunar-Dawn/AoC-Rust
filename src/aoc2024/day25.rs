use crate::runner;
use crate::util::DynResult;

runner!();

type Key = [u8; 5];
type Lock = [u8; 5];
type ParsedData = (Vec<Lock>, Vec<Key>);
fn parse(input: &str) -> DynResult<ParsedData> {
    let mut lines = input.lines().peekable();

    let mut keys = Vec::new();
    let mut locks = Vec::new();

    while let Some(_) = lines.peek() {
        let first_line = lines.next().unwrap();
        if first_line.starts_with('#') {
            let mut lock = [u8::MAX; 5];

            for height in 0..6 {
                let line = lines.next().unwrap();
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        continue;
                    }
                    lock[i] = lock[i].min(height);
                }
            }
            locks.push(lock);
        } else {
            let mut key = [0; 5];

            for height in 0..6 {
                let line = lines.next().unwrap();
                for (i, c) in line.chars().enumerate() {
                    if c == '.' {
                        continue;
                    }
                    key[i] = key[i].max(5 - height);
                }
            }
            keys.push(key);
        }
        lines.next();
    }

    Ok((locks, keys))
}

fn key_can_fit_lock(key: &Key, lock: &Lock) -> bool {
    for i in 0..5 {
        if lock[i] + key[i] > 5 {
            return false;
        }
    }
    true
}

fn part1((locks, keys): &ParsedData) -> u64 {
    let mut potential_keys = 0;

    for lock in locks {
        for key in keys {
            if key_can_fit_lock(key, lock) {
                potential_keys += 1;
            }
        }
    }

    potential_keys
}
fn part2(_: &ParsedData) -> &str {
    "Merry Christmas!"
}
