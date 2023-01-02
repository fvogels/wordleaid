use crate::{judgments::{WordJudgment, Word, LetterJudgment}, util::letter_index};

use super::Judge;


pub struct FastJudge<const N: usize> {

}

impl<const N: usize> FastJudge<N> {
    pub fn new() -> Self {
        FastJudge { }
    }
}

impl<const N: usize> Judge<N> for FastJudge<N> {
    fn judge(&self, judged: &Word<N>, goal: &Word<N>) -> WordJudgment<N> {
        let mut used = [false; N];
        let mut freqs = [0; 26];
        let mut judgment = WordJudgment { letters: [LetterJudgment::Incorrect; N] };

        for i in 0..N {
            let judged_letter = judged.letters[i];
            let goal_letter = goal.letters[i];

            if judged_letter == goal_letter {
                judgment.letters[i] = LetterJudgment::Correct;
                used[i] = true;
            } else {
                let index = letter_index(goal_letter);
                freqs[index] += 1;
            }
        }

        for i in 0..N {
            if judgment.letters[i] == LetterJudgment::Incorrect {
                let judged_letter_index = letter_index(judged.letters[i]);
                if freqs[judged_letter_index] > 0 {
                    judgment.letters[i] = LetterJudgment::Misplaced;
                    freqs[judged_letter_index] -= 1;
                }
            }
        }

        judgment
    }
}
