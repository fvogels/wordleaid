use interactive::Interactive;
use judging::FastJudge;
use judgments::{Word, WordJudgment};

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
    // let mut interactive = Interactive::new(words.into_iter());
    // interactive.guess("TARES", "M.M.M");

    // for s in interactive.possible_solutions().iter() {
    //     println!("{}", s);
    // }
    // println!("");

    // interactive.guess("HOIST", "..CMC");

    // for s in interactive.possible_solutions().iter() {
    //     println!("{}", s);
    // }
    // println!("");

    let all_words = ["AAAAA", "BBBBB", "CCCCC"].iter().map(|&x| Word::<5>::from_string(x));
    let go = GuessOptimizer::<5>::new(all_words.into_iter());
    let i = go.determine_best_guess(&vec![0, 1, 2], &vec![0]);
    println!("{}", i);
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{judging::{Judge, FastJudge}, judgments::WordJudgment};

    #[cfg(test)]
    use super::*;

    #[rstest]
    #[case("TRAIN", "TRAIN", "CCCCC")]
    #[case("TRAIN", "DRAIN", ".CCCC")]
    #[case("ABCDE", "EDCBA", "MMCMM")]
    #[case("ABCDE", "FGHIJ", ".....")]
    fn fast_judge(#[case] judged: &str, #[case] goal: &str, #[case] judgment_string: &str) {
        let judge = FastJudge::<5>::new();
        let judged = Word::from_string(judged);
        let goal = Word::from_string(goal);

        let actual = judge.judge(&judged, &goal);
        let expected = WordJudgment::<5>::parse(judgment_string);

        assert_eq!(expected, actual);
    }
}
