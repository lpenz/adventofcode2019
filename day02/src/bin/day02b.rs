// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day02::*;

const TARGET: usize = 19690720;

pub fn check(mut input: Vec<usize>, noun: usize, verb: usize) -> Result<bool> {
    input[1] = noun;
    input[2] = verb;
    Ok(calc(input)? == TARGET)
}

pub fn hunt(input0: Vec<usize>) -> Result<(usize, usize)> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if check(input0.clone(), noun, verb)? {
                return Ok((noun, verb));
            }
        }
    }
    Err(eyre!("noun-verb not found"))
}

fn process(input: &str) -> Result<usize> {
    let input = parser::parse(input)?;
    let (noun, verb) = hunt(input)?;
    Ok(noun * 100 + verb)
}

fn main() -> Result<()> {
    do_main(process)
}
