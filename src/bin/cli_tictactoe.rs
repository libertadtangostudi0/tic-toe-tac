// src/bin/cli_tictactoe.rs

use tictactoe::core::board::CoreBoard;
use tictactoe::core::player::CorePlayer;
use tictactoe::core::rules::check_winner;

use std::io::{self, Write};

fn main() {
    run_cli();
}

/// Simple CLI runner for CoreBoard logic.
/// This binary does not use Bevy and is meant only for debugging and testing.
fn run_cli() {
    let size: u8 = 3;
    let mut board = CoreBoard::new(size);
    let mut current_player = CorePlayer::X;

    println!("=== Tic Tac Toe CLI ===");
    println!("Board size: {} x {}", size, size);
    println!("Enter moves as: row col (0-based). Type 'q' to quit.\n");

    loop {
        print_board(&board);

        println!();
        println!(
            "Player {:?}, enter your move (row col) or 'q' to quit:",
            current_player
        );
        print!("> ");
        flush_stdout();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }

        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("q") {
            println!("Exit requested. Bye!");
            break;
        }

        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        if tokens.len() != 2 {
            println!("Please enter exactly two numbers: row col");
            continue;
        }

        let row: u8 = match tokens[0].parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid row. Use integer in range 0..{}", size - 1);
                continue;
            }
        };
        let col: u8 = match tokens[1].parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid col. Use integer in range 0..{}", size - 1);
                continue;
            }
        };

        if row >= size || col >= size {
            println!("Out of bounds. Use 0..{} for both row and col.", size - 1);
            continue;
        }

        let placed = board.place_mark(row, col, current_player);
        if !placed {
            println!("Cell ({}, {}) is already occupied. Try another one.", row, col);
            continue;
        }

        // Check game result after the move
        let result = check_winner(&board);
        if let Some(winner) = result.winner {
            print_board(&board);
            println!("\n=== Game over: winner = {:?} ===", winner);
            break;
        }

        if result.is_draw {
            print_board(&board);
            println!("\n=== Game over: draw ===");
            break;
        }

        // Switch player
        current_player = match current_player {
            CorePlayer::X => CorePlayer::O,
            CorePlayer::O => CorePlayer::X,
        };
    }
}

/// Print the current board to stdout.
fn print_board(board: &CoreBoard) {
    let size = board.size();

    println!();
    print!("   ");
    for col in 0..size {
        print!(" {} ", col);
    }
    println!();
    println!("  {}", "-".repeat((size as usize) * 3 + 1));

    for row in 0..size {
        print!("{} |", row);
        for col in 0..size {
            let ch = match board.get(row, col) {
                Some(CorePlayer::X) => 'X',
                Some(CorePlayer::O) => 'O',
                None => '.',
            };
            print!(" {} ", ch);
        }
        println!("|");
    }
    println!("  {}", "-".repeat((size as usize) * 3 + 1));
}

fn flush_stdout() {
    let _ = io::stdout().flush();
}
