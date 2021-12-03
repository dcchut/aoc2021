use crate::{Digits, ProblemInput, Solution};

pub struct Q3;

/// Returns an iterator over the `col`th element of each entry in `items`.
fn iter_col<T: Copy>(items: &[Vec<T>], col: usize) -> impl Iterator<Item = T> + '_ {
    items.iter().map(move |row| row[col])
}

/// Returns whether the number of ones in `digits` is greater than or equal to
/// the number of zeroes..
fn ones_geq_zeroes<I: IntoIterator<Item = u32>>(digits: I) -> bool {
    let (mut zeroes, mut ones) = (0, 0);

    for item in digits {
        if item == 0 {
            zeroes += 1;
        } else if item == 1 {
            ones += 1;
        }
    }

    ones >= zeroes
}

fn parse(lines: &ProblemInput) -> Vec<Vec<u32>> {
    lines
        .parse::<Vec<Digits>>()
        .into_iter()
        .map(|d| d.digits)
        .collect()
}

fn filter(mut items: Vec<Vec<u32>>, flip: bool) -> i32 {
    // Iterate through each column until there is one item remaining.
    for col in 0..items[0].len() {
        if items.len() == 1 {
            break;
        }

        // Shrink `items` down to only those items having the specified digit
        // in this column.
        let keep_ones = ones_geq_zeroes(iter_col(&items, col)) ^ flip;
        items.retain(|row| row[col] == keep_ones as u32)
    }

    assert_eq!(items.len(), 1);

    // Convert the one remaining item from a vec of binary digits to a decimal number
    items[0].iter().fold(0, |acc, &v| 2 * acc + (v as i32))
}

impl Solution for Q3 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let digits = parse(lines);

        // This is both hilarious and terrible.
        let (gamma, epsilon) = (0..digits[0].len())
            .map(|col| ones_geq_zeroes(iter_col(&digits, col)))
            .fold((0, 0), |(gamma, epsilon), digit| {
                (
                    (gamma * 2) + digit as u32,
                    (epsilon * 2) + (1 - digit as u32),
                )
            });

        (gamma * epsilon).to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let digits = parse(lines);

        let oxygen = filter(digits.clone(), false);
        let scrubber = filter(digits, true);

        (oxygen * scrubber).to_string()
    }
}
