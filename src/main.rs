mod game;

use std::io::{self, Write};

use game::state::GameState;

fn main() {
    let mut game = GameState::new();

    println!("Rust Football Manager");
    println!("=====================");

    while game.running {
        println!();
        println!("Season: {}", game.season);
        println!("Day: {}", game.current_day);

        println!();
        println!("1. Continue");
        println!("2. View game state");
        println!("3. Quit");

        println!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "1" => {
                game.advance_day();
                println!("Advance to day {}.", game.current_day);
            }

            "2" => {
                println!("{:#?}", game);
            }

            "3" => {
                game.quit();
                println!("Goodbye.");
            }

            _ => {
                println!("Unknown command.");
            }
        }
    }
}