#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LetterJudgment {
    Incorrect,
    Correct,
    Misplaced,
}

impl LetterJudgment {
    pub fn to_int(&self) -> usize {
        match self {
            LetterJudgment::Incorrect => 0,
            LetterJudgment::Correct => 1,
            LetterJudgment::Misplaced => 2,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct WordJudgment<const N: usize> {
    pub letters: [LetterJudgment; N],
}

impl<const N: usize> WordJudgment<N> {
    pub fn new() -> Self {
        WordJudgment {
            letters: [LetterJudgment::Incorrect; N],
        }
    }

    pub fn parse(string: &str) -> Option<Self> {
        if string.len() != N {
            None
        } else {
            let mut result = WordJudgment::new();
            let letters: Vec<_> = string.chars().collect();

            for i in 0..N {
                result.letters[i] = match letters[i] {
                    'X' => LetterJudgment::Incorrect,
                    'C' => LetterJudgment::Correct,
                    'M' => LetterJudgment::Misplaced,
                    _ => return None,
                }
            }

            Some(result)
        }
    }

    pub fn to_int(&self) -> usize {
        self.letters.iter().fold(0, |acc, k| acc * 3 + k.to_int())
    }

    pub const fn max_int_value() -> usize {
        3usize.pow(N as u32)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word<const N: usize> {
    pub letters: [char; N],
}

impl<const N: usize> Word<N> {
    pub fn from_string(string: &str) -> Result<Self, String> {
        if string.len() != N {
            Err(format!("Invalid word length: {} has length {}", string, string.len()))
        } else {
            let mut result = [' '; N];
            let letters: Vec<_> = string.chars().collect();

            for i in 0..N {
                result[i] = letters[i];
            }

            Ok(Word { letters: result })
        }
    }

    pub fn to_string(&self) -> String {
        self.letters.iter().collect()
    }
}
