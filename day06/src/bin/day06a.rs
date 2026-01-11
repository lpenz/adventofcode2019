// Copyright (C) 2026 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashMap;

use day06::*;

fn count(g: &HashMap<Node, Vec<Node>>, lvl: usize, node: &Node) -> usize {
    if let Some(children) = g.get(node) {
        lvl + children
            .iter()
            .map(|child| count(g, lvl + 1, child))
            .sum::<usize>()
    } else {
        lvl
    }
}

fn process(input: &str) -> Result<usize> {
    let input = parser::parse(input)?;
    let mut g = HashMap::<Node, Vec<Node>>::default();
    for (a, b) in input {
        g.entry(a).and_modify(|e| e.push(b)).or_insert(vec![b]);
    }
    Ok(count(&g, 0, &Node::from_str("COM").unwrap()))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 42);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
