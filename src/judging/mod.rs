use crate::judgments::{Word, WordJudgment};

mod fastjudge;
mod slowjudge;

pub trait Judge<const N: usize> {
    fn judge(&self, judged: &Word<N>, goal: &Word<N>) -> WordJudgment<N>;
}

pub use fastjudge::FastJudge;
pub use slowjudge::SlowJudge;
