use interactive::Interactive;
use judgments::Word;
use std::io;

mod interactive;
mod judging;
mod judgments;
mod optimizer;
mod util;

fn read_word_list<const N: usize>(path: &str) -> Vec<Word<N>> {
    let contents = std::fs::read_to_string(path).unwrap();

    contents.split("\n").map(|s| Word::<N>::from_string(strip_newline(s).as_str()).unwrap()).collect()
}

fn strip_newline<'a>(string: &'a str) -> String {
    string.chars().rev().skip_while(|c| c.is_whitespace()).collect::<String>().chars().rev().collect()
}

fn ask_string(prompt: Option<&str>) -> Result<String, String> {
    if let Some(prompt) = prompt {
        println!("{}", prompt);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| e.to_string());

    Ok(strip_newline(input.as_str()).to_owned())
}

fn main() {
    let words = read_word_list::<5>("words.txt");
    let mut interactive = Interactive::new(words.into_iter());

    loop {
        match interactive.solution() {
            Some(solution) => {
                println!("Only one solution left: {}", solution);
            }
            None => {
                println!("Possible solutions: {}", interactive.possible_solution_count());
                println!("Best guess: {}", interactive.find_optimal_guess().to_string());
            }
        }

        let choice = ask_string(Some("Enter command [?glqr]:")).unwrap();

        match choice.as_str() {
            "?" => {
                println!("g: Guess word");
                println!("r: Reset");
                println!("l: Show list of possible solutions");
                println!("q: Quit");
            }
            "g" => {
                let guess = ask_string(Some("Enter guess:")).unwrap().to_ascii_uppercase();
                let judgment = ask_string(Some("Enter judgment:")).unwrap().to_ascii_uppercase();

                match interactive.guess(guess.as_str(), judgment.as_str()) {
                    Ok(_) => { }
                    Err(err) => { println!("Error: {}", err); }
                }
            }
            "l" => {
                for solution in interactive.possible_solutions() {
                    println!("{}", solution);
                }
            }
            "r" => {
                interactive.reset();
            }
            "q" => {
                break;
            }
            _ => {
                println!("Invalid command :-(");
            }
        }
    }
}
