use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use crate::util::parse::ScanError::NotEnoughNumbers;
use crate::util::DynResult;

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

#[derive(Debug)]
pub enum ScanError {
    NotEnoughNumbers(usize),
}
impl Display for ScanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotEnoughNumbers(i) => write!(f, "Not enough numbers parsed, could only read {i}"),
        }
    }
}
impl Error for ScanError {}

pub fn scan_integers<T>(mut s: &str) -> Vec<T>
where
    T: FromStr<Err = ParseIntError> + num_traits::PrimInt + num_traits::ConstZero,
{
    let mut ret = Vec::new();

    while !s.is_empty() {
        let take = match take_integers(s) {
            Ok(t) => t,
            Err(NotEnoughNumbers(..)) => break,
        };

        s = take.0;
        let [x] = take.1;

        ret.push(x);
    }

    ret
}
pub fn take_integers<const N: usize, T>(mut s: &str) -> Result<(&str, [T; N]), ScanError>
where
    T: FromStr<Err = ParseIntError> + num_traits::PrimInt + num_traits::ConstZero,
{
    let mut ret = [T::ZERO; N];

    for i in 0..N {
        let Some(first_to_parse) = s.find(|c: char| c.is_digit(10)) else {
            return Err(NotEnoughNumbers(i));
        };

        s = &s[first_to_parse..];

        let parse_end = *&s.find(|c: char| !c.is_digit(10)).unwrap_or(s.len());

        ret[i] = *&s[..parse_end].parse::<T>().unwrap();
        s = &s[parse_end..];
    }

    Ok((s, ret))
}

pub fn scan_integers_signed<T>(mut s: &str) -> Vec<T>
where
    T: FromStr<Err = ParseIntError>
        + num_traits::PrimInt
        + num_traits::ConstZero
        + num_traits::Signed,
{
    let mut ret = Vec::new();

    while !s.is_empty() {
        let take = match take_integers_signed(s) {
            Ok(t) => t,
            Err(NotEnoughNumbers(..)) => break,
        };

        s = take.0;
        let [x] = take.1;

        ret.push(x);
    }

    ret
}
pub fn take_integers_signed<const N: usize, T>(mut s: &str) -> Result<(&str, [T; N]), ScanError>
where
    T: FromStr<Err = ParseIntError>
        + num_traits::PrimInt
        + num_traits::ConstZero
        + num_traits::Signed,
{
    let mut ret = [T::ZERO; N];

    for i in 0..N {
        let Some(first_to_parse) = signed_find_parse_start(s) else {
            return Err(NotEnoughNumbers(i));
        };

        s = &s[first_to_parse..];
        let parse_end = *&s[1..]
            .find(|c: char| !c.is_digit(10))
            .unwrap_or(s.len() - 1)
            + 1;

        ret[i] = *&s[..parse_end].parse::<T>().unwrap();
        s = &s[parse_end..];
    }

    Ok((s, ret))
}

fn signed_find_parse_start(mut s: &str) -> Option<usize> {
    loop {
        let Some(first_to_parse) = s.find(|c: char| c == '-' || c.is_digit(10)) else {
            return None;
        };

        s = &s[first_to_parse..];
        let mut chars = s.chars();

        match chars.next() {
            None => unreachable!(),
            Some('-') => {}
            _ => return Some(first_to_parse),
        }
        let c = match chars.next() {
            None => return None,
            Some(c) => c,
        };

        if c.is_digit(10) {
            return Some(first_to_parse);
        }
        s = &s[1..]
    }
}
