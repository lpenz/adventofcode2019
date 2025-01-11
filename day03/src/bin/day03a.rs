// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day03::*;

use sqrid::postrait::PosT;

use std::collections::HashMap;
use std::collections::HashSet;

fn process(input: &str) -> Result<i32> {
    let input = parser::parse(input)?;
    let mut paths = HashMap::<(i32, i32), HashSet<usize>>::default();
    for (i, wire) in input.into_iter().enumerate() {
        let mut p = (0_i32, 0_i32);
        paths.entry(p).or_default().insert(i);
        for (dir, num) in wire.into_iter() {
            for _ in 0..num {
                p = (p + (dir)).unwrap();
                paths.entry(p).or_default().insert(i);
            }
        }
    }
    paths
        .into_iter()
        .filter(|(p, wires)| *p != (0, 0) && wires.len() >= 2)
        .map(|(p, _)| p.x().abs() + p.y().abs())
        .min()
        .ok_or_eyre("no crosses found")
}

#[test]
fn test1() -> Result<()> {
    assert_eq!(process(EXAMPLE1)?, 6);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    assert_eq!(process(EXAMPLE2)?, 159);
    Ok(())
}

#[test]
fn test3() -> Result<()> {
    assert_eq!(process(EXAMPLE3)?, 135);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
