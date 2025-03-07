// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use rayon::prelude::*;

use day04::*;

fn process(input: &str) -> Result<usize> {
    let (min, max) = parser::parse(input)?;
    Ok((min..=max).into_par_iter().filter(valid::<false>).count())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 2);
    Ok(())
}

#[test]
fn test_valid() {
    assert!(valid::<false>(&111111));
    assert!(!valid::<false>(&223450));
    assert!(!valid::<false>(&123789));
}

fn main() -> Result<()> {
    do_main(process)
}
