use crate::{
    event::Event,
    tile::{generate_tiles, guess_is_correct, Guess},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunningState {
    Running,
    Win,
    Loss,
}

#[derive(Clone, Debug)]
pub struct Model {
    guesses: Vec<Guess>,
    word: String,
    state: RunningState,
}

fn determine_state(guesses: &Vec<Guess>) -> RunningState {
    if guesses.len() < 5 {
        return RunningState::Running;
    }

    match guess_is_correct(guesses.last().unwrap()) {
        true => RunningState::Win,
        false => RunningState::Loss,
    }
}

impl Model {
    pub fn new(word: String) -> Self {
        Model {
            guesses: Vec::new(),
            word,
            state: RunningState::Running,
        }
    }

    pub fn make_guess(&self, word: &String) -> Self {
        let guesses: Vec<Guess> = self
            .guesses
            .to_owned()
            .into_iter()
            .chain([generate_tiles(word, &self.word)])
            .collect();

        Model {
            state: determine_state(&guesses),
            guesses,
            word: self.word.to_owned(),
        }
    }

    pub fn get_guesses(&self) -> &Vec<Guess> {
        &self.guesses
    }

    pub fn get_word(&self) -> &String {
        &self.word
    }

    pub fn get_state(&self) -> &RunningState {
        &self.state
    }

    pub fn update(&self, event: Box<dyn Event>) -> Model {
        event.command(self)
    }
}
