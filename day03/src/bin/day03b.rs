// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day03::*;

use std::collections::HashMap;

fn process(input: &str) -> Result<usize> {
    let input = parser::parse(input)?;
    let mut paths = HashMap::<(i32, i32), HashMap<usize, usize>>::default();
    for (i, wire) in input.into_iter().enumerate() {
        let mut p = (0_i32, 0_i32);
        let mut steps = 0;
        paths.entry(p).or_default().insert(i, 0);
        for (dir, num) in wire.into_iter() {
            for _ in 0..num {
                p = (p + (dir)).unwrap();
                steps += 1;
                paths.entry(p).or_default().insert(i, steps);
            }
        }
    }
    paths
        .into_iter()
        .filter_map(|(p, wiresmap)| {
            (p != (0, 0) && wiresmap.len() >= 2).then_some(wiresmap.into_values().sum())
        })
        .min()
        .ok_or_eyre("no crosses found")
}

#[test]
fn test1() -> Result<()> {
    assert_eq!(process(EXAMPLE1)?, 30);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    assert_eq!(process(EXAMPLE2)?, 610);
    Ok(())
}

#[test]
fn test3() -> Result<()> {
    assert_eq!(process(EXAMPLE3)?, 410);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
