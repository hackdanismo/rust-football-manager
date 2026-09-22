use crate::models::{
    club::Club,
    league::League,
    player::{Player, Position},
};

#[derive(Debug)]
pub struct GameState {
    pub current_day: u32,
    pub season: u32,
    pub running: bool,
    pub league: League,
    pub clubs: Vec<Club>,
}

impl GameState {
    pub fn new() -> Self {
        let mut club = Club::new(1, "Eastleigh Town");
        let mut clubTwo = Club::new(2, "Winchester City");
        let mut clubThree = Club::new(3, "Southampton Atheletics");
        let mut clubFour = Club::new(4, "Hampshire Rovers");

        club.add_player(Player::new(
            1,
            "James Walker",
            27,
            Position::Goalkeeper,
            42,
            12,
            35,
            51,
        ));

        club.add_player(Player::new(
            2,
            "Ben Harris",
            24,
            Position::Defender,
            58,
            31,
            72,
            67,
        ));

        club.add_player(Player::new(
            3,
            "Jack Hughes",
            22,
            Position::Midfielder,
            74,
            61,
            53,
            70,
        ));

        club.add_player(Player::new(
            4,
            "Ryan Carter",
            25,
            Position::Forward,
            55,
            78,
            28,
            76,
        ));

        let league = League::new(
            1, "Southern Premier", vec![1, 2, 3, 4],
        );

        Self {
            current_day: 1,
            season: 2026,
            running: true,
            clubs: vec![club, clubTwo, clubThree, clubFour],
            league,
        }
    }

    pub fn advance_day(&mut self) {
        self.current_day += 1;
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}