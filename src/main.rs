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

fn show_league_table(game: &GameState) {
    println!();
    println!("{}", game.league.name);
    println!("==============================================");
    println!("Club                     P   W   D   L   GF  GA  Pts");

    for entry in &game.league.table {
        let club = game
            .clubs
            .iter()
            .find(|club| club.id == entry.club_id)
            .expect("Club not found");

        println!(
            "{:<24} {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>3}",
            club.name,
            entry.played,
            entry.won,
            entry.drawn,
            entry.lost,
            entry.goals_for,
            entry.goals_against,
            entry.points,
        );
    }
}

fn show_fixtures(game: &GameState) {
    println!();
    println!("Fixtures");
    println!("==============================================");

    for fixture in &game.fixtures {
        let home_club = game
            .clubs
            .iter()
            .find(|club| club.id == fixture.home_club_id)
            .expect("Home club not found");

        let away_club = game
            .clubs
            .iter()
            .find(|club| club.id == fixture.away_club_id)
            .expect("Away club not found");

        if fixture.played {
            println!(
                "#{:<3} {:<24} {} - {} {}",
                fixture.id,
                home_club.name,
                fixture.home_goals.unwrap_or(0),
                fixture.away_goals.unwrap_or(0),
                away_club.name,
            );
        } else {
            println!(
                "#{:<3} {:<24} vs {}",
                fixture.id,
                home_club.name,
                away_club.name,
            );
        }
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
        println!("4. View league table");
        println!("5. View fixtures");
        println!("6. Quit");

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "1" => {
                game.advance_day();
                println!("Advanced to day {}.", game.current_day);
            }

            "2" => {
                println!("{:#?}", game);
            }

            "3" => {
                show_squad(&game);
            }

            "4" => {
                show_league_table(&game);
            }

            "5" => {
                show_fixtures(&game);
            }

            "6" => {
                game.quit();
                println!("Goodbye.");
            }

            _ => {
                println!("Unknown command.");
            }
        }
    }
}