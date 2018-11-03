mod tile;
mod board;
mod add;
mod move_tile;
mod state;

use state::State;

const DIMENSION: usize = 4;
const FOOR_PROBABILITY: f32 = 0.2;

pub struct Action {
    pub action_type: String,
    pub direction: Option<usize>,
    pub random_value: Option<f32>,
    pub random_position: Option<f32>,
}

pub fn reducer(action: Action) -> State {
    State::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reducer_works() {
        let action = Action {
            action_type: String::from("init"),
            direction: None,
            random_value: None,
            random_position: None,
        };
        let state = reducer(action);

        assert_eq!(state.changed, false);
    }
}
