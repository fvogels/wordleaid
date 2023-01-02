use crate::{judgments::{Word, WordJudgment}, judging::{Judge, FastJudge}};

pub struct GuessOptimizer<const N: usize> {
    pub words: Vec<Word<N>>,
    matrix: Vec<Vec<u64>>,
}

impl<const N: usize> GuessOptimizer<N> {
    pub fn new(words: impl Iterator<Item=Word<N>>) -> Self {
        let words: Vec<_> = words.collect();
        let judge = FastJudge::<N>::new();
        let mut matrix = Vec::from_iter((0..words.len()).map(|_| vec![0; words.len()]));

        for (igoal, goal) in words.iter().enumerate() {
            for (ijudged, judged) in words.iter().enumerate() {
                let judgment = judge.judge(judged, goal);
                matrix[ijudged][igoal] = judgment.to_int();
            }
        }

        GuessOptimizer { words, matrix }
    }

    fn judge(&self, guess: u64, goal: u64) -> u64 {
        self.matrix[guess as usize][goal as usize]
    }

    pub fn determine_best_guess(&self, guesses: &Vec<u64>, goals: &Vec<u64>) -> u64 {
        *guesses.iter().min_by_key(|&guess| (self.evaluate_guess(*guess, goals) * 1000000f64) as u64).unwrap()
    }

    fn evaluate_guess(&self, guess: u64, goals: &Vec<u64>) -> f64 {
        let mut table = vec![0f64; WordJudgment::<N>::max_int_value() as usize];

        for goal in goals {
            let judgment = self.judge(guess, *goal);
            table[judgment as usize] += 1f64;
        }

        let mut result = table.iter().fold(0f64, |acc, k| acc + k * k);

        result / (goals.len() as f64)
    }
}
