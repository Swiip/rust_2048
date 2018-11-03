mod tile;
mod board;
mod add;
mod move_tile;

const DIMENSION: usize = 4;
const FOOR_PROBABILITY: f32 = 0.2;

pub fn run() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        run();
    }
}
