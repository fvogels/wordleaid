use judging::{Judge, FastJudge};
use judgments::{Word, WordJudgment};

mod judgments;
mod judging;
mod util;


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
        let judge = FastJudge::<N>::new();
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
        let judge = FastJudge::<5>::new();
        let judged = Word::from_string(judged);
        let goal = Word::from_string(goal);

        let actual = judge.judge(&judged, &goal);
        let expected = WordJudgment::<5>::parse(judgment_string);

        assert_eq!(expected, actual);
    }
}