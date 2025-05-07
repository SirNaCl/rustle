use std::io::stdin;

use crate::{
    event::{Event, EventError, EventReset, EventSubmit},
    model::{Model, RunningState},
};

pub trait ViewBackend {
    fn new() -> Self;
    fn draw_model(&self, model: &Model);
    fn show_error(&self, err: &EventError);
    fn draw_restart_prompt(&self);
}

pub struct View<T>
where
    T: ViewBackend,
{
    backend: T,
}

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Could not read stdin");
    input.trim_end_matches('\n').into()
}

impl<T> View<T>
where
    T: ViewBackend,
{
    pub fn new(backend: T) -> Self {
        View { backend }
    }

    fn await_event_running(&self) -> Box<dyn Event> {
        let e = loop {
            match EventSubmit::new(get_input()) {
                Ok(event) => break event,
                Err(e) => self.backend.show_error(&e),
            }
        };

        Box::new(e)
    }

    fn await_event_stopped(&self) -> Box<dyn Event> {
        self.backend.draw_restart_prompt();
        Box::new(EventReset {})
    }

    pub fn await_event(&self, state: &RunningState) -> Box<dyn Event> {
        match state {
            RunningState::Running => self.await_event_running(),
            RunningState::Win | RunningState::Loss => self.await_event_stopped(),
        }
    }

    pub fn draw_model(&self, model: &Model) {
        self.backend.draw_model(model)
    }
}
