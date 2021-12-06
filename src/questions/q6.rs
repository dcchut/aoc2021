use crate::{FromProblemInput, ProblemInput, Solution};

pub struct Q6;

#[derive(Debug)]
struct Pond {
    fish: [usize; 9],
}

impl Pond {
    fn tick(&mut self) {
        let zeroes = self.fish[0];
        for i in 1..9 {
            self.fish[i - 1] = self.fish[i];
        }
        self.fish[8] = zeroes;
        self.fish[6] += zeroes;
    }

    fn size(&self) -> usize {
        self.fish.iter().sum()
    }
}

impl FromProblemInput for Pond {
    fn from(lines: &ProblemInput) -> Self {
        let mut fish = [0; 9];
        for time in lines.lines[0].split(',') {
            fish[time.parse::<usize>().unwrap()] += 1;
        }
        Pond { fish }
    }
}

impl Solution for Q6 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let mut pond: Pond = lines.parse();

        for _ in 0..80 {
            pond.tick();
        }

        pond.size().to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let mut pond: Pond = lines.parse();

        for _ in 0..256 {
            pond.tick();
        }

        pond.size().to_string()
    }
}
