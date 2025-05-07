mod backends;
mod event;
mod model;
mod tile;
mod view;
mod wordlist;

use backends::cli::BackendCLI;
use model::Model;
use view::{View, ViewBackend};
use wordlist::wordlist_get_rnd;

fn main() {
    let mut model = Model::new(wordlist_get_rnd());
    let view = View::new(BackendCLI::new());

    loop {
        view.draw_model(&model);
        let event = view.await_event(model.get_state());
        model = model.update(event);
    }
}
