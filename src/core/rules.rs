// src/core/rules.rs

use crate::core::board::CoreBoard;
use crate::core::player::CorePlayer;

pub struct CoreGameResult {
    pub winner: Option<CorePlayer>,
    pub is_draw: bool,
}

pub fn check_winner(board: &CoreBoard) -> CoreGameResult {
    let n = board.size() as usize;
    let mut winner: Option<CorePlayer> = None;
    let mut has_empty = false;

    // Rows
    for row in 0..n {
        let first = board.get(row as u8, 0);
        if let Some(player) = first {
            let mut all_same = true;
            for col in 1..n {
                if board.get(row as u8, col as u8) != Some(player) {
                    all_same = false;
                    break;
                }
            }
            if all_same {
                winner = Some(player);
                break;
            }
        }
    }

    // Columns
    if winner.is_none() {
        for col in 0..n {
            let first = board.get(0, col as u8);
            if let Some(player) = first {
                let mut all_same = true;
                for row in 1..n {
                    if board.get(row as u8, col as u8) != Some(player) {
                        all_same = false;
                        break;
                    }
                }
                if all_same {
                    winner = Some(player);
                    break;
                }
            }
        }
    }

    // Main diagonal
    if winner.is_none() {
        let first = board.get(0, 0);
        if let Some(player) = first {
            let mut all_same = true;
            for i in 1..n {
                if board.get(i as u8, i as u8) != Some(player) {
                    all_same = false;
                    break;
                }
            }
            if all_same {
                winner = Some(player);
            }
        }
    }

    // Anti-diagonal
    if winner.is_none() {
        let first = board.get(0, (n - 1) as u8);
        if let Some(player) = first {
            let mut all_same = true;
            for i in 1..n {
                let row = i as u8;
                let col = (n - 1 - i) as u8;
                if board.get(row, col) != Some(player) {
                    all_same = false;
                    break;
                }
            }
            if all_same {
                winner = Some(player);
            }
        }
    }

    // Check empties for draw detection
    for row in 0..n {
        for col in 0..n {
            if board.get(row as u8, col as u8).is_none() {
                has_empty = true;
                break;
            }
        }
        if has_empty {
            break;
        }
    }

    CoreGameResult {
        winner,
        is_draw: winner.is_none() && !has_empty,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::player::CorePlayer;

    #[test]
    fn horizontal_win() {
        let mut b = CoreBoard::new(3);
        b.set(1, 0, Some(CorePlayer::X));
        b.set(1, 1, Some(CorePlayer::X));
        b.set(1, 2, Some(CorePlayer::X));
        let r = check_winner(&b);
        assert_eq!(r.winner, Some(CorePlayer::X));
        assert!(!r.is_draw);
    }

    #[test]
    fn main_diagonal_win() {
        let mut b = CoreBoard::new(3);
        b.set(0, 0, Some(CorePlayer::O));
        b.set(1, 1, Some(CorePlayer::O));
        b.set(2, 2, Some(CorePlayer::O));
        let r = check_winner(&b);
        assert_eq!(r.winner, Some(CorePlayer::O));
        assert!(!r.is_draw);
    }

    #[test]
    fn draw_full_board() {
        let mut b = CoreBoard::new(3);
        // X O X
        // X O O
        // O X X
        use CorePlayer::{O, X};
        b.set(0, 0, Some(X));
        b.set(0, 1, Some(O));
        b.set(0, 2, Some(X));
        b.set(1, 0, Some(X));
        b.set(1, 1, Some(O));
        b.set(1, 2, Some(O));
        b.set(2, 0, Some(O));
        b.set(2, 1, Some(X));
        b.set(2, 2, Some(X));

        let r = check_winner(&b);
        assert_eq!(r.winner, None);
        assert!(r.is_draw);
    }
}
