use board::Board;

#[derive(Debug)]
pub struct State {
    pub board: Board,
    pub changed: bool,
    pub won: bool,
    pub lost: bool,
}

impl State {
    pub fn new() -> State {
        State {
            board: Board::new(),
            changed: false,
            won: false,
            lost: false
        }
    }
}
