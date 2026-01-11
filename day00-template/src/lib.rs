// Copyright (C) 2026 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "0\n";

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    // use super::*;

    pub fn all<'src>() -> impl Parser<'src, &'src str, Vec<u8>, extra::Err<Rich<'src, char>>> {
        let line = number().then_ignore(just('\n'));
        line.repeated().collect()
    }

    pub fn parse(input: &str) -> Result<Vec<u8>> {
        chumsky_parse(input, all())
    }

    #[test]
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(input.len(), 1);
        Ok(())
    }
}
