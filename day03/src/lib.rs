// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;
use sqrid::dir::Dir;

pub const EXAMPLE1: &str = "R8,U5,L5,D3\nU7,R6,D4,L4\n";

pub const EXAMPLE2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
";

pub const EXAMPLE3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
";

pub type Wire = Vec<(Dir, i32)>;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn dirlen(input: &str) -> IResult<&str, (Dir, i32)> {
        let (input, dir) = combinator::map(character::one_of("URDL"), |c| match c {
            'U' => Dir::N,
            'R' => Dir::E,
            'D' => Dir::S,
            'L' => Dir::W,
            _ => unreachable!(),
        })(input)?;
        let (input, dist) = character::i32(input)?;
        Ok((input, (dir, dist)))
    }

    fn wire(input: &str) -> IResult<&str, Wire> {
        let (input, wire) = multi::separated_list1(bytes::tag(","), dirlen)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, wire))
    }

    pub fn parse(input: &str) -> Result<Vec<Wire>> {
        aoc::parse_with!(multi::many1(wire), input)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE1)?;
    assert_eq!(input.len(), 2);
    Ok(())
}
