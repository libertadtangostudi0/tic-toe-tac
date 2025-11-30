#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CorePlayer {
    X,
    O,
}

impl CorePlayer {
    pub fn other(self) -> CorePlayer {
        match self {
            CorePlayer::X => CorePlayer::O,
            CorePlayer::O => CorePlayer::X,
        }
    }
}
