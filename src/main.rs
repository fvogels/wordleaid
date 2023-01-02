use judgments::Word;

use crate::optimizer::GuessOptimizer;

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
    let indices: Vec<u64> = (0..words.len()).map(|x| x as u64).collect();
    let guesser = GuessOptimizer::<5>::new(words.into_iter());

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
