// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "1,9,10,3,2,3,11,0,99,30,40,50\n";

pub mod parser {
    use aoc::parser::*;

    // use super::*;

    fn num(input: &str) -> IResult<&str, usize> {
        let (input, i) = character::u32(input)?;
        Ok((input, i as usize))
    }

    fn line(input: &str) -> IResult<&str, Vec<usize>> {
        let (input, nums) = multi::separated_list1(tag(","), num)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, nums))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<usize>> {
        aoc::parse_with!(line, bufin)
    }
}

pub fn calc(mut input: Vec<usize>) -> Result<usize> {
    let mut pc = 0;
    loop {
        let instr = input[pc];
        if instr == 99 {
            break;
        }
        let v1 = input[input[pc + 1]];
        let v2 = input[input[pc + 2]];
        let o = input[pc + 3];
        if instr == 1 {
            input[o] = v1 + v2;
        } else if instr == 2 {
            input[o] = v1 * v2;
        }
        pc += 4;
    }
    Ok(input[0])
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 12);
    Ok(())
}
