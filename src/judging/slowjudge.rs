use crate::judgments::{LetterJudgment, Word, WordJudgment};

use super::Judge;

pub struct SlowJudge<const N: usize> {}

impl<const N: usize> SlowJudge<N> {
    pub fn new() -> Self {
        SlowJudge {}
    }
}

impl<const N: usize> Judge<N> for SlowJudge<N> {
    fn judge(&self, judged: &Word<N>, goal: &Word<N>) -> WordJudgment<N> {
        let mut used = [false; N];
        let mut judgment = WordJudgment {
            letters: [LetterJudgment::Incorrect; N],
        };

        for i in 0..N {
            if judged.letters[i] == goal.letters[i] {
                judgment.letters[i] = LetterJudgment::Correct;
                used[i] = true;
            }
        }

        for i in 0..N {
            if judgment.letters[i] == LetterJudgment::Incorrect {
                for j in 0..N {
                    if !used[j] && judged.letters[i] == goal.letters[j] {
                        judgment.letters[i] = LetterJudgment::Misplaced;
                        used[j] = true;
                    }
                }
            }
        }

        judgment
    }
}
