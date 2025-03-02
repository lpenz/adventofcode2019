// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use rayon::prelude::*;

use day04::*;

fn process(input: &str) -> Result<usize> {
    let (min, max) = parser::parse(input)?;
    Ok((min..=max).into_par_iter().filter(valid::<true>).count())
}

#[test]
fn test_valid() {
    assert!(!valid::<true>(&111111));
    assert!(!valid::<true>(&223450));
    assert!(!valid::<true>(&123789));
    assert!(valid::<true>(&112233));
    assert!(!valid::<true>(&123444));
    assert!(valid::<true>(&111122));
}

fn main() -> Result<()> {
    do_main(process)
}
