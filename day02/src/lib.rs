// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "1,9,10,3,2,3,11,0,99,30,40,50\n";

pub mod parser {
    use aoc::parser::*;

    // use super::*;

    fn num(input: &str) -> IResult<&str, u32> {
        character::u32(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<u32>> {
        let (input, nums) = multi::separated_list1(tag(","), num)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, nums))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<u32>> {
        aoc::parse_with!(line, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 12);
    Ok(())
}
