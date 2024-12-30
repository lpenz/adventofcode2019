// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day01::*;

fn process(input: &str) -> Result<i32> {
    let input = parser::parse(input)?;
    Ok(input.into_iter().map(|i| i / 3 - 2).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 34241);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
