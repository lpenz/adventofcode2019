// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day01::*;

fn process(bufin: impl BufRead) -> Result<i32> {
    let input = parser::parse(bufin)?;
    Ok(input.into_iter().map(|i| i / 3 - 2).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 34241);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
