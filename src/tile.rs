pub type Guess = [Tile; 5];

#[derive(Clone, Debug, PartialEq)]
pub enum TileStatus {
    Correct,
    Misplaced,
    Wrong,
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub status: TileStatus,
    pub letter: char,
}

pub fn guess_is_correct(guess: &Guess) -> bool {
    guess.into_iter().all(|g| g.status == TileStatus::Correct)
}

fn create_tile(char_guess: &char, index: usize, correct: &String) -> Tile {
    let letter = char_guess.to_owned();
    let status = match correct.find(letter.to_ascii_lowercase()) {
        Some(i) => {
            if i == index {
                TileStatus::Correct
            } else {
                TileStatus::Misplaced
            }
        }
        None => TileStatus::Wrong,
    };

    Tile { status, letter }
}

pub fn generate_tiles(guess: &String, correct: &String) -> Guess {
    guess
        .char_indices()
        .map(|(ind, c)| create_tile(&c, ind, correct))
        .collect::<Vec<Tile>>()
        .try_into()
        .unwrap()
}
