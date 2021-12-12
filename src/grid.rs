use crate::{FromProblemInput, FromProblemInputLine, ProblemInput};
use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    #[inline(always)]
    pub fn left(self) -> Self {
        match self {
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
        }
    }

    #[inline(always)]
    pub fn right(self) -> Self {
        // the compiler will take care of it
        self.left().left().left()
    }

    #[inline(always)]
    pub fn all() -> HashSet<Self> {
        let mut all = HashSet::new();
        all.insert(Direction::Left);
        all.insert(Direction::Right);
        all.insert(Direction::Up);
        all.insert(Direction::Down);

        all
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "L" {
            Ok(Direction::Left)
        } else if s == "R" {
            Ok(Direction::Right)
        } else if s == "U" {
            Ok(Direction::Up)
        } else if s == "D" {
            Ok(Direction::Down)
        } else {
            Err(anyhow!(format!("couldn't convert {} to Direction", s)))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Movement {
    direction: Direction,
    steps: usize,
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let chars = s.chars().collect::<Vec<_>>();

        if !chars.is_empty() {
            // first character should be R, L, U, or D
            let direction = Direction::from_str(&chars[0].to_string())?;
            let steps: usize = (&s[1..]).parse()?;

            Ok(Movement { direction, steps })
        } else {
            Err(anyhow!(format!("failed to create Movement from {}", s)))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn go(&self, direction: Direction) -> Self {
        let delta_x = match direction {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        };

        let delta_y = match direction {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0,
        };

        Self::new(self.x + delta_x, self.y + delta_y)
    }

    pub fn l1(&self) -> i64 {
        fn abs(u: i64) -> i64 {
            if u < 0 {
                -u
            } else {
                u
            }
        }

        abs(self.x) + abs(self.y)
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

pub trait GridVisitor {
    type Output;

    /// Called when a move occurs
    fn go(&mut self, pos: Position, direction: Direction, movement: Movement, step_index: usize);

    /// Called after all movement is done
    fn process(self: Box<Self>) -> Self::Output;
}

/// A struct that keeps track of what positions our grid has visited
#[derive(Debug, Clone, Default)]
pub struct HistoryVisitor {
    history: HashSet<Position>,
}

impl HistoryVisitor {
    pub fn new() -> Self {
        Self {
            history: HashSet::new(),
        }
    }
}

impl GridVisitor for HistoryVisitor {
    type Output = HashSet<Position>;

    fn go(
        &mut self,
        pos: Position,
        _direction: Direction,
        _movement: Movement,
        _step_index: usize,
    ) {
        self.history.insert(pos);
    }

    fn process(self: Box<Self>) -> HashSet<Position> {
        self.history
    }
}

/// A struct that keeps track of how many steps you've made the first time you reach
/// a point.
#[derive(Debug, Clone, Default)]
pub struct StepVisitor {
    step_count: usize,
    step_map: HashMap<Position, usize>,
}

impl StepVisitor {
    pub fn new() -> Self {
        Self {
            step_count: 0,
            step_map: HashMap::new(),
        }
    }
}

impl GridVisitor for StepVisitor {
    type Output = HashMap<Position, usize>;

    fn go(
        &mut self,
        pos: Position,
        _direction: Direction,
        _movement: Movement,
        _step_index: usize,
    ) {
        self.step_count += 1;

        self.step_map.entry(pos).or_insert(self.step_count);
    }

    fn process(self: Box<Self>) -> Self::Output {
        self.step_map
    }
}

pub struct Grid<T> {
    pub pos: Position,
    pub visitor: Box<dyn GridVisitor<Output = T>>,
}

impl<T> Grid<T> {
    pub fn new<V: 'static + GridVisitor<Output = T>>(visitor: V) -> Self {
        Self {
            pos: Position::default(),
            visitor: Box::new(visitor),
        }
    }

    pub fn go_many(mut self, movements: Vec<Movement>) -> T {
        for movement in movements {
            self.go(movement);
        }

        self.visitor.process()
    }

    pub fn go(&mut self, movement: Movement) {
        // process our movement
        for step_index in 0..movement.steps {
            // for each step, move.
            self.pos = self.pos.go(movement.direction);

            // then call our visitor
            self.visitor
                .go(self.pos, movement.direction, movement, step_index);
        }
    }
}

impl FromProblemInputLine for Vec<Movement> {
    fn from_line(line: &str) -> Self {
        let mut current_line = Vec::new();

        // split the line at commas
        for part in line.split(',') {
            // part is R31 or something like that
            let movement = Movement::from_str(part).expect("invalid movement string");

            current_line.push(movement);
        }

        current_line
    }
}

impl FromProblemInput<'_> for (Vec<Movement>, Vec<Movement>) {
    fn from(lines: &ProblemInput) -> Self {
        let mut input = lines.parse::<Vec<Vec<Movement>>>();

        let f = input.pop().expect("invalid movement string");
        let g = input.pop().expect("invalid movement string");

        (f, g)
    }
}
