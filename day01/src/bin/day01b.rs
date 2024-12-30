// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day01::*;

fn mass2fuel(mass: i32) -> i32 {
    let mut fuel = 0;
    let mut m = mass;
    while m > 0 {
        let f = m / 3 - 2;
        if f > 0 {
            fuel += f;
        }
        m = f;
    }
    fuel
}

fn process(input: &str) -> Result<i32> {
    let input = parser::parse(input)?;
    Ok(input.into_iter().map(mass2fuel).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 51316);
    Ok(())
}

#[test]
fn test_mass2fuel() -> Result<()> {
    assert_eq!(mass2fuel(14), 2);
    assert_eq!(mass2fuel(1969), 966);
    assert_eq!(mass2fuel(100756), 50346);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
