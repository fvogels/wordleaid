use interactive::Interactive;
use judging::FastJudge;
use judgments::{Word, WordJudgment};
use std::io;

use crate::optimizer::GuessOptimizer;

mod interactive;
mod judging;
mod judgments;
mod optimizer;
mod util;

fn read_word_list<const N: usize>(path: &str) -> Vec<Word<N>> {
    let contents = std::fs::read_to_string(path).unwrap();

    contents.split("\n").map(Word::<N>::from_string).collect()
}

fn main() {
    let words = read_word_list::<5>("words.txt");
    let mut interactive = Interactive::new(words.into_iter());
    let stdin = io::stdin();

    loop {
        match interactive.solution() {
            Some(solution) => {
                println!("Only one solution left: {}", solution);
            }
            None => {
                println!("Possible solutions: {}", interactive.possible_solution_count());
                println!("Best guess: {}", interactive.find_optimal_guess().to_string());
            }
        }

        println!("Enter guess:");
        let mut guess = String::new();
        stdin.read_line(&mut guess).unwrap();
        guess = guess.to_ascii_uppercase();
        let guess = guess.strip_suffix("\r\n").or_else(|| { guess.strip_suffix("\n") }).unwrap();

        println!("Enter jugdment: (X=wrong, M=misplaced, C=correct)");
        let mut judgment = String::new();
        stdin.read_line(&mut judgment).unwrap();
        judgment = judgment.to_ascii_uppercase();
        let judgment = judgment.strip_suffix("\r\n").or_else(|| { judgment.strip_suffix("\n") }).unwrap();

        interactive.guess(guess, judgment);
    }
}
