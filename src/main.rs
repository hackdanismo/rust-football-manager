mod game;
mod models;

use std::io::{self, Write};

use game::state::GameState;

fn show_squad(game: &GameState) {
    let club = &game.clubs[0];

    println!();
    println!("Squad: {}", club.name);
    println!("===============================");

    for player in &club.players {
        println!(
            "{} | {:?} | Age {} | PAS {} SHO {} TAC {} PAC {}",
            player.name,
            player.position,
            player.age,
            player.passing,
            player.shooting,
            player.tackling,
            player.pace,
        );
    }
}

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
        println!("3. View squad");
        println!("4. Quit");

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
                show_squad(&game);
            }

            "4" => {
                game.quit();
                println!("Goodbye.");
            }

            _ => {
                println!("Unknown command.");
            }
        }
    }
}