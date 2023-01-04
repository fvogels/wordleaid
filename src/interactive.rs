use crate::{optimizer::GuessOptimizer, judgments::{Word, WordJudgment}};

pub struct Interactive<const N: usize> {
    optimizer: GuessOptimizer<N>,
    candidates: Vec<usize>,
}

impl<const N: usize> Interactive<N> {
    pub fn new(words: impl Iterator<Item=Word<N>>) -> Self {
        let optimizer = GuessOptimizer::new(words);
        let candidates = optimizer.word_indices().collect();

        Self::in_progress(optimizer, candidates)
    }

    fn in_progress(optimizer: GuessOptimizer<N>, candidates: Vec<usize>) -> Self {
        Interactive { optimizer, candidates }
    }

    pub fn find_optimal_guess(&self) -> Word<N> {
        let optimal_index = self.optimizer.determine_best_guess(&self.optimizer.word_indices().collect(), &self.candidates);
        self.optimizer.find_word_by_index(optimal_index as usize)
    }

    pub fn guess(&mut self, guessed: &str, judgment: &str) {
        let guessed = self.optimizer.find_word_index(&Word::<N>::from_string(guessed));
        let judgment =  WordJudgment::<N>::parse(judgment).to_int();
        self.candidates.retain(|c| self.optimizer.judge(guessed, *c) == judgment);
    }

    pub fn solution(&self) -> Option<String> {
        if self.candidates.len() == 1 {
            let index = self.candidates[0];
            let solution = self.optimizer.find_word_by_index(index);
            Some(solution.to_string())
        } else {
            None
        }
    }

    pub fn possible_solutions(&self) -> Vec<String> {
        self.candidates.iter().map(|&idx| self.optimizer.find_word_by_index(idx).to_string()).collect()
    }

    pub fn possible_solution_count(&self) -> usize {
        self.candidates.len()
    }

    pub fn reset(&mut self) {
        self.candidates = self.optimizer.word_indices().collect();
    }
}
