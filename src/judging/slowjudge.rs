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


#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{judging::{Judge, FastJudge}, judgments::WordJudgment};

    #[cfg(test)]
    use super::*;

    #[rstest]
    #[case("TRAIN", "TRAIN", "CCCCC")]
    #[case("TRAIN", "DRAIN", "XCCCC")]
    #[case("ABCDE", "EDCBA", "MMCMM")]
    #[case("ABCDE", "FGHIJ", "XXXXX")]
    fn judge(#[case] judged: &str, #[case] goal: &str, #[case] judgment_string: &str) {
        let judge = SlowJudge::<5>::new();
        let judged = Word::from_string(judged);
        let goal = Word::from_string(goal);

        let actual = judge.judge(&judged, &goal);
        let expected = WordJudgment::<5>::parse(judgment_string);

        assert_eq!(expected, actual);
    }
}