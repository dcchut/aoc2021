use crate::{FromProblemInputLine, ProblemInput, Solution};
use itertools::Itertools;
use num::Integer;
use std::ops::Add;

pub struct Q18;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    LBracket,
    RBracket,
    Comma,
    Value(i64),
}

impl Token {
    fn as_value(self) -> Option<i64> {
        if let Token::Value(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ',' => Token::Comma,
            x => Token::Value(x.to_digit(10).unwrap() as i64),
        }
    }
}

struct LinearTreeIter<'a> {
    tokens: &'a [Token],
    index: usize,
    l_depth: usize,
    r_depth: usize,
}

#[derive(Copy, Clone, Debug)]
struct ValueRef {
    value: i64,
    index: usize,
    l_depth: usize,
    r_depth: usize,
}

impl ValueRef {
    fn total_depth(self) -> usize {
        self.l_depth + self.r_depth
    }
}

impl<'a> Iterator for LinearTreeIter<'a> {
    type Item = ValueRef;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&token) = self.tokens.get(self.index) {
            self.index += 1;
            match token {
                Token::LBracket => {
                    self.l_depth += 1;
                }
                Token::Comma => {
                    self.l_depth -= 1;
                    self.r_depth += 1;
                }
                Token::RBracket => {
                    self.r_depth -= 1;
                }
                Token::Value(value) => {
                    return Some(ValueRef {
                        value,
                        index: self.index - 1,
                        l_depth: self.l_depth,
                        r_depth: self.r_depth,
                    })
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct LinearTree {
    tokens: Vec<Token>,
}

impl LinearTree {
    fn magnitude(&self) -> i64 {
        self.value_iter()
            .map(|v| 3_i64.pow(v.l_depth as u32) * 2_i64.pow(v.r_depth as u32) * v.value)
            .sum()
    }

    fn resolve(&mut self) {
        'rl: loop {
            if let Some(v) = self.value_iter().find(|v| v.total_depth() > 4) {
                self.explode(v.index);
                continue 'rl;
            }
            if let Some(v) = self.value_iter().find(|v| v.value >= 10) {
                self.split(v.index);
                continue 'rl;
            }
            break;
        }
    }

    fn value_iter(&self) -> LinearTreeIter<'_> {
        LinearTreeIter {
            tokens: &self.tokens,
            index: 0,
            l_depth: 0,
            r_depth: 0,
        }
    }

    fn explode(&mut self, index: usize) {
        if let Some(left) = (0..index)
            .rev()
            .find(|&i| self.tokens[i].as_value().is_some())
        {
            self.tokens[left] = Token::Value(
                self.tokens[left].as_value().unwrap() + self.tokens[index].as_value().unwrap(),
            );
        }
        if let Some(right) =
            (index + 3..self.tokens.len()).find(|i| self.tokens[*i].as_value().is_some())
        {
            self.tokens[right] = Token::Value(
                self.tokens[right].as_value().unwrap() + self.tokens[index + 2].as_value().unwrap(),
            );
        }
        self.tokens.splice(index - 1..=index + 3, [Token::Value(0)]);
    }

    fn split(&mut self, index: usize) {
        let v = self.tokens[index].as_value().unwrap();
        self.tokens.splice(
            index..=index,
            [
                Token::LBracket,
                Token::Value(v.div_floor(&2)),
                Token::Comma,
                Token::Value(v.div_ceil(&2)),
                Token::RBracket,
            ],
        );
    }
}

impl FromProblemInputLine for LinearTree {
    fn from_line(line: &str) -> Self {
        LinearTree {
            tokens: line.chars().map(Token::from).collect(),
        }
    }
}

impl Add for &LinearTree {
    type Output = LinearTree;

    fn add(self, rhs: Self) -> Self::Output {
        let mut tokens = Vec::with_capacity(self.tokens.len() + rhs.tokens.len() + 3);
        tokens.push(Token::LBracket);
        tokens.extend(&self.tokens);
        tokens.push(Token::Comma);
        tokens.extend(&rhs.tokens);
        tokens.push(Token::RBracket);

        let mut tree = LinearTree { tokens };
        tree.resolve();
        tree
    }
}

impl Add for LinearTree {
    type Output = LinearTree;

    fn add(self, rhs: Self) -> Self::Output {
        (&self) + (&rhs)
    }
}

impl Solution for Q18 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let linear_trees: Vec<LinearTree> = lines.parse();
        linear_trees
            .into_iter()
            .reduce(Add::add)
            .unwrap()
            .magnitude()
            .to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let linear_trees: Vec<LinearTree> = lines.parse();
        linear_trees
            .iter()
            .permutations(2)
            .map(|p| (p[0] + p[1]).magnitude())
            .max()
            .unwrap()
            .to_string()
    }
}
