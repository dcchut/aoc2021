use anyhow::Result;
use aoc2020::questions::*;
use aoc2020::{ProblemInput, Solution};
use crossbeam::thread;

fn run_problem(solutions: &'static [Box<dyn Solution>], index: usize) -> Result<(String, String)> {
    let (part1, part2) = thread::scope(move |s| {
        let solution = &solutions[index];

        let part1 = s
            .spawn(move |_| {
                let path = format!("data/q{}.txt", index + 1);
                let problem_input = ProblemInput::new(path).unwrap();

                solution.part1(&problem_input)
            })
            .join();

        let part2 = s
            .spawn(move |_| {
                let path = format!("data/q{}.txt", index + 1);
                let problem_input = ProblemInput::new(path).unwrap();

                solution.part2(&problem_input)
            })
            .join();

        (part1, part2)
    })
    .unwrap();

    Ok((part1.unwrap(), part2.unwrap()))
}

fn main() -> Result<()> {
    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(Q1 {}),
        Box::new(Q2 {}),
        Box::new(Q3 {}),
        Box::new(Q4 {}),
        Box::new(Q5 {}),
        Box::new(Q6 {}),
        Box::new(Q7 {}),
        Box::new(Q8 {}),
        Box::new(Q9 {}),
        Box::new(Q10 {}),
        Box::new(Q11 {}),
        Box::new(Q12 {}),
        Box::new(Q13 {}),
        Box::new(Q14 {}),
        Box::new(Q15 {}),
        Box::new(Q16 {}),
        Box::new(Q17 {}),
        Box::new(Q18 {}),
        Box::new(Q19 {}),
        Box::new(Q20 {}),
        Box::new(Q21 {}),
        Box::new(Q22 {}),
        Box::new(Q23 {}),
        Box::new(Q24 {}),
        Box::new(Q25 {}),
    ];

    let solutions: &'static [Box<dyn Solution>] = Box::leak(solutions.into_boxed_slice());

    let reply = rprompt::prompt_reply_stdout("Problem: ")?;

    if let Ok(index) = reply.parse::<usize>() {
        let index = index - 1;

        if index < solutions.len() {
            let (part1, part2) = run_problem(solutions, index)?;

            println!("Part 1: {}", part1);
            println!("Part 2: {}", part2);
        }
    }

    Ok(())
}
