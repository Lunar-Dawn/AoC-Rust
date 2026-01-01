use crate::dyn_result::DynResult;

pub fn parse_range(s: &str) -> DynResult<(u64, u64)> {
    let mut parts = s.split('-');

    let Some(lower) = parts.next() else {
        return Err(format!("Missing lower part of range: {}", s).into());
    };
    let Some(upper) = parts.next() else {
        return Err(format!("Missing upper part of range: {}", s).into());
    };
    Ok((lower.parse()?, upper.parse()?))
}
