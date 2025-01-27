// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day04::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Secret {
    pub digits: Vec<u8>,
    pub max: Num,
}

impl Secret {
    pub fn new(min: Num, max: Num) -> Self {
        let mut digits = format!("{}", min)
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();
        for i in 1..digits.len() {
            if digits[i] < digits[i - 1] {
                digits[i] = digits[i - 1];
            }
        }
        Secret { digits, max }
    }

    pub fn valid(&self) -> bool {
        let mut hasdouble = false;
        for (i, &d) in self.digits.iter().enumerate().skip(1) {
            match d.cmp(&self.digits[i - 1]) {
                std::cmp::Ordering::Less => {
                    return false;
                }
                std::cmp::Ordering::Equal => {
                    hasdouble = true;
                }
                std::cmp::Ordering::Greater => {}
            }
        }
        hasdouble
    }
}

impl Iterator for Secret {
    type Item = Secret;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.clone();
        let mut i = next.digits.len() - 1;
        loop {
            let mut min = (0..i)
                .map(|i| self.digits[i])
                .max()
                .unwrap_or(self.digits[0]);
            if next.digits[i] < 9 {
                next.digits[i] += 1;
                min = std::cmp::max(min, next.digits[i]);
                for j in (i + 1)..next.digits.len() {
                    next.digits[j] = min;
                }
                if !next.valid() {
                    continue;
                }
                if Num::from(&next) <= self.max {
                    *self = next.clone();
                    return Some(next);
                } else {
                    return None;
                }
            }
            next.digits[i] = min;
            i -= 1;
        }
    }
}

impl From<&Secret> for Num {
    fn from(s: &Secret) -> Self {
        s.digits.iter().fold(0, |a, c| a * 10 + *c as Num)
    }
}

#[test]
fn test_secret() -> Result<()> {
    let max = 999999;
    let mut s = Secret::new(111111, max);
    assert_eq!(s.next(), Some(Secret::new(111112, max)));
    assert_eq!(s.next(), Some(Secret::new(111113, max)));
    let mut s = Secret::new(111999, max);
    assert_eq!(s.next(), Some(Secret::new(112222, max)));
    let mut s = Secret::new(123455, max);
    assert_eq!(s.next(), Some(Secret::new(123466, max)));
    assert_eq!(s.next(), Some(Secret::new(123477, max)));
    let mut s = Secret::new(199999, max);
    assert_eq!(s.next(), Some(Secret::new(222222, max)));
    Ok(())
}

fn process(input: &str) -> Result<usize> {
    let (min, max) = parser::parse(input)?;
    let s = Secret::new(min, max);
    Ok(s.count() + 1)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 2);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
