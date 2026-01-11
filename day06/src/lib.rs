// Copyright (C) 2026 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use std::str::FromStr;

pub use aoc::*;

use tinystr::TinyAsciiStr;

pub const EXAMPLE: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node(pub TinyAsciiStr<3>);

impl FromStr for Node {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.parse().wrap_err("could not convert to TinyAsciiStr")?,
        ))
    }
}

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    pub fn parse(input: &str) -> Result<Vec<(Node, Node)>> {
        let node = none_of(" )\n")
            .repeated()
            .at_least(1)
            .collect::<String>()
            .try_map(do_parse);
        chumsky_parse(
            input,
            node.then_ignore(just(")"))
                .then(node)
                .then_ignore(just("\n"))
                .repeated()
                .collect::<Vec<(Node, Node)>>(),
        )
    }

    #[test]
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(input.len(), 11);
        Ok(())
    }
}
