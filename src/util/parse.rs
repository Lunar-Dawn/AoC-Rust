use crate::util::DynResult;
use std::str::FromStr;

pub fn parse_split_char<T, C>(s: &str, delim: char) -> Result<C, T::Err>
where
    T: FromStr,
    C: FromIterator<T>,
{
    s.split(delim)
        .map(|s| s.parse::<T>())
        .collect::<Result<_, _>>()
}
pub fn parse_split_ws<T, C>(s: &str) -> Result<C, T::Err>
where
    T: FromStr,
    C: FromIterator<T>,
{
    s.split_whitespace()
        .map(|s| s.parse::<T>())
        .collect::<Result<_, _>>()
}

pub fn parse_range(s: &str) -> DynResult<(u64, u64)> {
    let v: Vec<_> = parse_split_char(s, '-')?;

    let Some(lower) = v.get(0) else {
        return Err(format!("Missing lower part of range: {}", s).into());
    };
    let Some(upper) = v.get(1) else {
        return Err(format!("Missing upper part of range: {}", s).into());
    };
    Ok((*lower, *upper))
}
