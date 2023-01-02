use crate::judgments::{Word, WordJudgment};

mod slowjudge;
mod fastjudge;


pub trait Judge<const N: usize> {
    fn judge(&self, judged: &Word<N>, goal: &Word<N>) -> WordJudgment<N>;
}

pub use slowjudge::SlowJudge;
pub use fastjudge::FastJudge;