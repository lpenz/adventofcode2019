// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day02::*;

fn calc(mut input: Vec<usize>) -> Result<usize> {
    let mut pc = 0;
    loop {
        let instr = input[pc];
        if instr == 99 {
            break;
        }
        let v1 = input[input[pc + 1]];
        let v2 = input[input[pc + 2]];
        let o = input[pc + 3];
        if instr == 1 {
            input[o] = v1 + v2;
        } else if instr == 2 {
            input[o] = v1 * v2;
        }
        pc += 4;
    }
    Ok(input[0])
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut input = parser::parse(bufin)?;
    input[1] = 12;
    input[2] = 2;
    calc(input)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(calc(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])?, 3500);
    Ok(())
}

#[test]
fn othertests() -> Result<()> {
    assert_eq!(calc(vec![1, 0, 0, 0, 99])?, 2);
    assert_eq!(calc(vec![2, 3, 0, 3, 99])?, 2);
    assert_eq!(calc(vec![2, 4, 4, 5, 99, 0])?, 2);
    assert_eq!(calc(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])?, 30);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
