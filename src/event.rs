use std::{borrow::Borrow, process::exit};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    model::Model,
    wordlist::{wordlist_contains, wordlist_get_rnd},
};

#[derive(Debug)]
pub enum EventError {
    WordLength,
    WordUnknown,
}

pub trait Event {
    fn command(&self, model: &Model) -> Model;
}

pub struct EventSubmit {
    guess: String,
}

impl EventSubmit {
    pub fn new(word: String) -> Result<Self, EventError> {
        if word.graphemes(true).count() != 5 {
            return Err(EventError::WordLength);
        }

        if !wordlist_contains(word.borrow()) {
            return Err(EventError::WordUnknown);
        }

        Ok(EventSubmit { guess: word })
    }
}

impl Event for EventSubmit {
    fn command(&self, model: &Model) -> Model {
        model.make_guess(self.guess.borrow())
    }
}

pub struct EventReset {}

impl Event for EventReset {
    fn command(&self, _model: &Model) -> Model {
        Model::new(wordlist_get_rnd())
    }
}

pub struct EventQuit {}

impl Event for EventQuit {
    fn command(&self, _model: &Model) -> Model {
        exit(0)
    }
}
