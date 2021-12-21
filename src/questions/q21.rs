use crate::{FromProblemInput, ProblemInput, Solution};
use std::cmp::{min, Ordering};
use std::collections::HashMap;

pub struct Q21;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Pair(i64, i64);

impl FromProblemInput<'_> for Pair {
    fn from(lines: &ProblemInput) -> Self {
        let positions: Vec<i64> = lines.parse();
        Pair(positions[1], positions[3])
    }
}

trait Die {
    fn roll(&mut self) -> Vec<i64>;
    fn roll_count(&self) -> i64 {
        0
    }
}

#[derive(Clone, Debug, Default)]
struct DeterministicDie {
    current: i64,
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> Vec<i64> {
        let result = self.current + 1;
        self.current += 1;
        vec![result]
    }

    fn roll_count(&self) -> i64 {
        self.current
    }
}

#[derive(Clone, Debug, Default)]
struct QuantumDie;

impl Die for QuantumDie {
    fn roll(&mut self) -> Vec<i64> {
        vec![1, 2, 3]
    }
}

fn _wrap(x: i64) -> i64 {
    ((x - 1) % 10) + 1
}

#[derive(Clone, Debug)]
struct State {
    states: HashMap<(Pair, Pair), i64>,
    terminal: HashMap<Pair, i64>,
    threshold: i64,
}

impl State {
    fn new(pos: Pair, threshold: i64) -> Self {
        let states = HashMap::from([((pos, Pair::default()), 1)]);
        State {
            states,
            terminal: HashMap::new(),
            threshold,
        }
    }

    fn step<D: Die>(&mut self, die: &mut D) -> bool {
        let states = std::mem::take(&mut self.states);

        for ((pos, score), count) in states.into_iter() {
            for a in die.roll() {
                for b in die.roll() {
                    for c in die.roll() {
                        let p1_pos = _wrap(pos.0 + a + b + c);
                        let p1_score = score.0 + p1_pos;

                        if p1_score >= self.threshold {
                            *self.terminal.entry(Pair(p1_score, score.1)).or_default() += count;
                            continue;
                        }

                        for u in die.roll() {
                            for v in die.roll() {
                                for w in die.roll() {
                                    let p2_pos = _wrap(pos.1 + u + v + w);
                                    let p2_score = score.1 + p2_pos;
                                    if p2_score >= self.threshold {
                                        *self
                                            .terminal
                                            .entry(Pair(p1_score, p2_score))
                                            .or_default() += count;
                                    } else {
                                        *self
                                            .states
                                            .entry((Pair(p1_pos, p2_pos), Pair(p1_score, p2_score)))
                                            .or_default() += count;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        self.states.is_empty()
    }
}

impl Solution for Q21 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let mut state = State::new(lines.parse(), 1000);
        let mut die = DeterministicDie::default();
        while !state.step(&mut die) {}

        (state.terminal.keys().next().map(|p| min(p.0, p.1)).unwrap() * die.roll_count())
            .to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let mut state = State::new(lines.parse(), 21);
        let mut die = QuantumDie::default();
        while !state.step(&mut die) {}

        state
            .terminal
            .into_iter()
            .fold([0, 0], |[p1_wins, p2_wins], (p, c)| {
                match p.0.cmp(&p.1) {
                    Ordering::Greater | Ordering::Equal => [p1_wins + c, p2_wins],
                    Ordering::Less => [p1_wins, p2_wins + c],
                }
            })
            .into_iter()
            .max()
            .unwrap()
            .to_string()
    }
}
