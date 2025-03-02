// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use itertools::Itertools;

pub use aoc::*;

pub const EXAMPLE: &str = "111110-111112\n";

pub type Num = u64;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn num(input: &str) -> PResult<&str, Num> {
        nom::error::context("cannot parse Num", map_res(character::u32, Num::try_from))(input)
    }

    fn line(input: &str) -> PResult<&str, (Num, Num)> {
        let (input, min) = context("min", num)(input)?;
        let (input, _) = tag("-")(input)?;
        let (input, max) = context("max", num)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (min, max)))
    }

    pub fn parse(input: &str) -> Result<(Num, Num)> {
        aoc::parse_with!(line, input)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE)?;
    assert_eq!(input, (111110, 111112));
    Ok(())
}

pub fn valid<const E: bool>(num: &Num) -> bool {
    let digits = format!("{}", num)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    if digits.iter().zip(digits.iter().skip(1)).any(|(a, b)| a > b) {
        return false;
    }
    digits
        .iter()
        .dedup_with_count()
        .any(|(c, _)| if E { c == 2 } else { c >= 2 })
}
