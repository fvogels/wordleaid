fn letter_index(ch: char) -> usize {
    (ch as usize) - ('A' as usize)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum LetterJudgment {
    Incorrect,
    Correct,
    Misplaced,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct WordJudgment<const N: usize> {
    letters: [LetterJudgment; N],
}

impl<const N: usize> WordJudgment<N> {
    fn new() -> Self {
        WordJudgment { letters: [LetterJudgment::Incorrect; N] }
    }

    fn parse(string: &str) -> Self {
        assert!(string.len() == N);

        let mut result = WordJudgment::new();
        let letters: Vec<_> = string.chars().collect();

        for i in 0..N {
            result.letters[i] = match letters[i] {
                '.' => LetterJudgment::Incorrect,
                'C' => LetterJudgment::Correct,
                'M' => LetterJudgment::Misplaced,
                _ => panic!("Invalid string"),
            }
        }

        result
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Word<const N: usize> {
    letters: [char; N]
}

impl<const N: usize> Word<N> {
    fn from_string(string: &str) -> Self {
        let mut result = [' '; N];
        let letters: Vec<_> = string.chars().collect();

        for i in 0..N {
            result[i] = letters[i];
        }

        Word { letters: result }
    }
}

trait Judge<const N: usize> {
    fn judge(&self, judged: &Word<N>, goal: &Word<N>) -> WordJudgment<N>;
}

struct SimpleJudge<const N: usize> {

}

impl<const N: usize> SimpleJudge<N> {
    pub fn new() -> Self {
        SimpleJudge { }
    }
}

impl<const N: usize> Judge<N> for SimpleJudge<N> {
    fn judge(&self, judged: &Word<N>, goal: &Word<N>) -> WordJudgment<N> {
        let mut used = [false; N];
        let mut judgment = WordJudgment { letters: [LetterJudgment::Incorrect; N] };

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

struct SimpleJudge2<const N: usize> {

}

impl<const N: usize> SimpleJudge2<N> {
    pub fn new() -> Self {
        SimpleJudge2 { }
    }
}

impl<const N: usize> Judge<N> for SimpleJudge2<N> {
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

fn read_word_list<const N: usize>(path: &str) -> Vec<Word<N>> {
    let contents = std::fs::read_to_string(path).unwrap();

    contents.split("\n").map(Word::<N>::from_string).collect()
}


fn main() {
    let words = read_word_list::<5>("words.txt");
    let judge = SimpleJudge2::<5>::new();
    let mut count = 0;

    for goal in words.iter() {
        for judged in words.iter() {
            judge.judge(judged, goal);
        }
    }

    println!("{count}");
}


#[cfg(test)]
mod test {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[rstest]
    #[case("TRAIN", "TRAIN", "CCCCC")]
    #[case("TRAIN", "DRAIN", ".CCCC")]
    #[case("ABCDE", "EDCBA", "MMCMM")]
    #[case("ABCDE", "FGHIJ", ".....")]
    fn simple_judge(#[case] judged: &str, #[case] goal: &str, #[case] judgment_string: &str) {
        let judge = SimpleJudge2::<5>::new();
        let judged = Word::from_string(judged);
        let goal = Word::from_string(goal);

        let actual = judge.judge(&judged, &goal);
        let expected = WordJudgment::<5>::parse(judgment_string);

        assert_eq!(expected, actual);
    }
}