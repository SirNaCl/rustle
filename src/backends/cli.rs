use crate::{
    tile::{Guess, Tile},
    view::ViewBackend,
};
use colored::{ColoredString, Colorize};

pub struct BackendCLI {}

const ERROR_LEN_STR: &str = "Invalid word length. Please enter a 5 letter word.";
const ERROR_WORD_INVALID_STR: &str = "Word not in wordlist";

fn colorize_tile(tile: &Tile) -> ColoredString {
    match tile.status {
        crate::tile::TileStatus::Correct => tile.letter.to_string().green().bold(),
        crate::tile::TileStatus::Misplaced => tile.letter.to_string().yellow(),
        crate::tile::TileStatus::Wrong => tile.letter.to_string().italic(),
    }
}

fn print_guess(guess: &Guess) {
    let text = guess
        .iter()
        .map(colorize_tile)
        .fold(String::new(), |s: String, cs: ColoredString| {
            s + &cs.to_string()
        });

    println!("{}", text)
}

fn print_guesses(guesses: &Vec<Guess>) {
    guesses.iter().for_each(print_guess);

    for _ in 0..(5 - guesses.len()) {
        println!("_____");
    }
}

impl ViewBackend for BackendCLI {
    fn new() -> Self {
        BackendCLI {}
    }

    fn draw_model(&self, model: &crate::model::Model) {
        print_guesses(model.get_guesses())
    }

    fn show_error(&self, err: &crate::event::EventError) {
        match err {
            crate::event::EventError::WordLength => println!("{}", ERROR_LEN_STR.red()),
            crate::event::EventError::WordUnknown => println!("{}", ERROR_WORD_INVALID_STR.red()),
        }
    }

    fn draw_restart_prompt(&self) {
        println!("Game finished, press enter to start new round")
    }
}
