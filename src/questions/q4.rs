use crate::{FromProblemInput, ProblemInput, Skip, Solution};

pub struct Q4;

struct BingoGame {
    draws: Vec<i64>,
    boards: Vec<Vec<Vec<i64>>>,
    completed_boards: Vec<Vec<Vec<i64>>>,
}

impl BingoGame {
    /// Returns a reference to the board that won most recently.
    fn most_recent_winner(&self) -> &Vec<Vec<i64>> {
        self.completed_boards.last().unwrap()
    }

    /// Has at least one board won?
    fn has_complete_boards(&self) -> bool {
        !self.completed_boards.is_empty()
    }

    /// Are there any boards that haven't won yet?
    fn has_incomplete_boards(&self) -> bool {
        !self.boards.is_empty()
    }

    /// Draws a new number, marking off any boards containing the number.  Returns the drawn number.
    fn draw(&mut self) -> i64 {
        fn is_winning_board(board: &[Vec<i64>]) -> bool {
            board.iter().any(|row| row.iter().sum::<i64>() == 0)
                || (0..board.len())
                    .any(|col| (0..board.len()).map(|row| board[row][col]).sum::<i64>() == 0)
        }

        fn mark(board: &mut Vec<Vec<i64>>, target: i64) {
            for row in board.iter_mut() {
                for val in row.iter_mut() {
                    if *val == target {
                        *val = 0;
                    }
                }
            }
        }

        let target = self.draws.pop().unwrap();
        for board in &mut self.boards {
            mark(board, target);
        }

        let completed_boards: Vec<_> = self
            .boards
            .drain_filter(|board| is_winning_board(board))
            .collect();
        self.completed_boards.extend(completed_boards);

        target
    }
}

impl FromProblemInput<'_> for BingoGame {
    fn from(lines: &ProblemInput) -> Self {
        let mut draws: Vec<i64> = lines.split(0..=0).parse();
        draws.reverse(); // reverse once so we're popping initial draws off the end

        // Now parse the boards
        let boards: Skip<Vec<Vec<i64>>> = lines.split(2..).parse();

        BingoGame {
            draws,
            boards: boards.unwrap(),
            completed_boards: Vec::new(),
        }
    }
}

fn sum(board: &[Vec<i64>]) -> i64 {
    board.iter().flatten().sum()
}

impl Solution for Q4 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let mut game: BingoGame = lines.parse();

        let mut last_drawn = 0;
        while !game.has_complete_boards() {
            last_drawn = game.draw();
        }

        let winning_board = game.most_recent_winner();
        (sum(winning_board) * last_drawn).to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let mut game: BingoGame = lines.parse();

        let mut last_drawn = 0;
        while game.has_incomplete_boards() {
            last_drawn = game.draw();
        }

        let winning_board = game.most_recent_winner();
        (sum(winning_board) * last_drawn).to_string()
    }
}
