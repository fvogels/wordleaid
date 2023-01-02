fn letter_index(ch: char) -> usize {
    (ch as usize) - ('A' as usize)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum LetterJudgment {
    Incorrect,
    Correct,
    Misplaced,
}

impl LetterJudgment {
    fn to_int(&self) -> u64 {
        match self {
            LetterJudgment::Incorrect => 0,
            LetterJudgment::Correct => 1,
            LetterJudgment::Misplaced => 2,
        }
    }
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

    fn to_int(&self) -> u64 {
        self.letters.iter().fold(0, |acc, k| acc * 3 + k.to_int())
    }

    const fn max_int_value() -> u64 {
        3u64.pow(N as u32)
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

struct Guesser<const N: usize> {
    words: Vec<Word<N>>,
    matrix: Vec<Vec<u64>>,
}

impl<const N: usize> Guesser<N> {
    fn new(words: impl Iterator<Item=Word<N>>) -> Self {
        let words: Vec<_> = words.collect();
        let judge = SimpleJudge2::<N>::new();
        let mut matrix = Vec::from_iter((0..words.len()).map(|_| vec![0; words.len()]));

        for (igoal, goal) in words.iter().enumerate() {
            for (ijudged, judged) in words.iter().enumerate() {
                let judgment = judge.judge(judged, goal);
                matrix[ijudged][igoal] = judgment.to_int();
            }
        }

        Guesser { words, matrix }
    }

    fn judge(&self, guess: u64, goal: u64) -> u64 {
        self.matrix[guess as usize][goal as usize]
    }

    fn determine_best_guess(&self, guesses: &Vec<u64>, goals: &Vec<u64>) -> u64 {
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

fn main() {
    let words = read_word_list::<5>("words.txt");
    let indices: Vec<u64> = (0..words.len()).map(|x| x as u64).collect();
    let guesser = Guesser::<5>::new(words.into_iter());

    let best_guess = guesser.determine_best_guess(&indices, &indices);
    let best_guess = guesser.words[best_guess as usize];

    println!("{:?}", best_guess);
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