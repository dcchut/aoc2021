use crate::{ProblemInput, Solution};
use from_iter::FromIterator;
use std::cmp::{max, min};

pub struct Q23;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token {
    A,
    B,
    C,
    D,
}

impl Token {
    const fn hallway(self) -> Position {
        match self {
            Token::A => Position::Hallway(2),
            Token::B => Position::Hallway(4),
            Token::C => Position::Hallway(6),
            Token::D => Position::Hallway(8),
        }
    }

    const fn multiplier(self) -> i64 {
        match self {
            Token::A => 1,
            Token::B => 10,
            Token::C => 100,
            Token::D => 1000,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Position {
    Hallway(usize),
    SideRoom(Token, usize),
}

impl Position {
    fn hallway(self) -> Position {
        match self {
            pos @ Position::Hallway(_) => pos,
            Position::SideRoom(token, _) => token.hallway(),
        }
    }

    fn is_side_room(self) -> bool {
        !matches!(self, Position::Hallway(_))
    }

    fn hallway_iter(self, dst: Position) -> impl Iterator<Item = Position> {
        if !matches!(self, Position::Hallway(_)) || !matches!(dst, Position::Hallway(_)) {
            panic!("can't call hallway_iter on a side room");
        }
        (min(self.index(), dst.index())..=max(self.index(), dst.index()))
            .filter(move |&i| i != self.index())
            .map(Position::Hallway)
    }

    fn side_room_above_iter(self) -> impl Iterator<Item = Position> {
        if matches!(self, Position::Hallway(_)) {
            panic!("can't call above on a hallway");
        }
        (0..self.index()).map(move |i| match self {
            Position::Hallway(_) => unreachable!(),
            Position::SideRoom(token, _) => Position::SideRoom(token, i),
        })
    }

    fn index(self) -> usize {
        match self {
            Position::Hallway(i) | Position::SideRoom(_, i) => i,
        }
    }
}

#[derive(Clone, Debug)]
pub struct State<const N: usize> {
    hallway: Hallway,
    side_rooms: [SideRoom<N>; 4],
}

#[derive(Clone, Debug, Default)]
pub struct Hallway([Option<Token>; 11]);

impl Hallway {
    pub fn iter(&self) -> impl Iterator<Item = (Position, Token)> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, token)| token.map(|token| (Position::Hallway(i), token)))
    }
}

impl<const N: usize> State<N> {
    fn new(tokens: [[Token; N]; 4]) -> Self {
        let _optionify =
            |x: [Token; N]| <[Option<Token>; N]>::from_iter(x.into_iter().map(Option::Some));

        let side_rooms = [
            SideRoom {
                base: Token::A,
                slots: _optionify(tokens[0]),
            },
            SideRoom {
                base: Token::B,
                slots: _optionify(tokens[1]),
            },
            SideRoom {
                base: Token::C,
                slots: _optionify(tokens[2]),
            },
            SideRoom {
                base: Token::D,
                slots: _optionify(tokens[3]),
            },
        ];
        Self {
            hallway: Hallway::default(),
            side_rooms,
        }
    }

    fn get_mut(&mut self, pos: Position) -> &mut Option<Token> {
        match pos {
            Position::Hallway(i) => &mut self.hallway.0[i],
            Position::SideRoom(Token::A, i) => &mut self.side_rooms[0].slots[i],
            Position::SideRoom(Token::B, i) => &mut self.side_rooms[1].slots[i],
            Position::SideRoom(Token::C, i) => &mut self.side_rooms[2].slots[i],
            Position::SideRoom(Token::D, i) => &mut self.side_rooms[3].slots[i],
        }
    }

    fn get(&self, pos: Position) -> Option<Token> {
        match pos {
            Position::Hallway(i) => self.hallway.0[i],
            Position::SideRoom(Token::A, i) => self.side_rooms[0].slots[i],
            Position::SideRoom(Token::B, i) => self.side_rooms[1].slots[i],
            Position::SideRoom(Token::C, i) => self.side_rooms[2].slots[i],
            Position::SideRoom(Token::D, i) => self.side_rooms[3].slots[i],
        }
    }

    fn swap(&mut self, src: Position, dst: Position) {
        let src = self.get_mut(src).take().unwrap();
        assert!(self.get_mut(dst).replace(src).is_none());
    }

    fn can_move(&self, mut src: Position, dst: Position) -> Option<i64> {
        if self.get(dst).is_some() {
            return None;
        }

        let mut distance = 0;
        if src.is_side_room() {
            // move into the hallway
            distance += src.index() as i64 + 1;

            for pos in src.side_room_above_iter() {
                if self.get(pos).is_some() {
                    return None;
                }
            }
            src = src.hallway();
            if self.get(src).is_some() {
                return None;
            }
        }

        let hallway_dst = match dst {
            pos @ Position::Hallway(_) => pos,
            Position::SideRoom(token, _) => token.hallway(),
        };

        for pos in src.hallway_iter(hallway_dst) {
            if self.get(pos).is_some() {
                return None;
            }
        }

        // move along the hallway
        distance += (src.index() as i64 - hallway_dst.index() as i64).abs();

        if dst.is_side_room() {
            // move down the side room
            distance += dst.index() as i64 + 1;

            for pos in dst.side_room_above_iter() {
                if self.get(pos).is_some() {
                    return None;
                }
            }
        }

        Some(distance)
    }

    fn solve(&mut self) -> i64 {
        self._solve(0, &mut 100000)
    }

    fn _solve(&mut self, acc: i64, best: &mut i64) -> i64 {
        if acc >= *best {
            return *best;
        }

        if self.side_rooms.iter().all(|room| room.is_done()) {
            *best = acc;
            return acc;
        }

        // Hallway -> side room
        for (pos, slot) in self.hallway.0.iter().enumerate() {
            if let Some(slot) = *slot {
                let pos = Position::Hallway(pos);
                let multiplier = slot.multiplier();
                for side_room in self.side_rooms.iter() {
                    if let Some(target_pos) = side_room.free_slot(slot) {
                        if let Some(steps) = self.can_move(pos, target_pos) {
                            let step_cost = multiplier * steps;
                            self.swap(pos, target_pos);
                            let result = self._solve(acc + step_cost, best);
                            self.swap(target_pos, pos);
                            return result;
                        }
                    }
                }
            }
        }

        // Side room -> hallway
        let mut potential_swaps = Vec::new();
        for side_room in self.side_rooms.iter() {
            for (pos, token) in side_room.iter() {
                if !side_room.is_locked(pos.index()) {
                    for i in 0..self.hallway.0.len() {
                        let target_pos = Position::Hallway(i);
                        if ![2, 4, 6, 8].contains(&i) {
                            if let Some(steps) = self.can_move(pos, target_pos) {
                                potential_swaps.push((pos, target_pos, steps * token.multiplier()));
                            }
                        }
                    }
                }
            }
        }

        for (src, dst, steps) in potential_swaps {
            self.swap(src, dst);
            self._solve(acc + steps, best);
            self.swap(dst, src);
        }

        *best
    }
}

#[derive(Clone, Debug)]
struct SideRoom<const N: usize> {
    base: Token,
    slots: [Option<Token>; N],
}

impl<const N: usize> SideRoom<N> {
    fn iter(&self) -> impl Iterator<Item = (Position, Token)> + '_ {
        self.slots.iter().enumerate().filter_map(|(i, token)| {
            if let Some(token) = *token {
                if !self.is_locked(i) {
                    return Some((Position::SideRoom(self.base, i), token));
                }
            }
            None
        })
    }

    fn is_done(&self) -> bool {
        self.slots.iter().all(|f| *f == Some(self.base))
    }

    fn is_locked(&self, index: usize) -> bool {
        self.slots[index..].iter().all(|f| {
            if let Some(f) = f {
                *f == self.base
            } else {
                false
            }
        })
    }

    fn free_slot(&self, token: Token) -> Option<Position> {
        if token != self.base {
            return None;
        }

        // First, find the highest index where we could place this token
        let mut slot_index = None;
        for (i, slot) in self.slots.iter().enumerate().rev() {
            if let Some(slot) = slot {
                if *slot != self.base {
                    return None;
                }
            } else {
                slot_index = Some(i);
                break;
            }
        }

        // Confirm that everything above that is empty.
        if let Some(i) = slot_index {
            if self.slots.iter().take(i).all(Option::is_none) {
                return Some(Position::SideRoom(self.base, i));
            }
        }

        None
    }
}

impl Solution for Q23 {
    fn part1(&self, _lines: &ProblemInput) -> String {
        State::new([
            [Token::A, Token::C],
            [Token::D, Token::D],
            [Token::A, Token::B],
            [Token::C, Token::B],
        ])
        .solve()
        .to_string()
    }

    fn part2(&self, _lines: &ProblemInput) -> String {
        State::new([
            [Token::A, Token::D, Token::D, Token::C],
            [Token::D, Token::C, Token::B, Token::D],
            [Token::A, Token::B, Token::A, Token::B],
            [Token::C, Token::A, Token::C, Token::B],
        ])
        .solve()
        .to_string()
    }
}
