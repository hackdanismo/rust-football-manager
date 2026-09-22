use crate::models::{
    club::Club,
    fixture::Fixture,
    league::League,
    player::{Player, Position},
};

#[derive(Debug)]
pub struct GameState {
    pub current_day: u32,
    pub season: u32,
    pub running: bool,
    pub fixtures: Vec<Fixture>,
    pub league: League,
    pub clubs: Vec<Club>,
}

impl GameState {
    pub fn new() -> Self {
        let mut club = Club::new(1, "Eastleigh Town");
        let club_two = Club::new(2, "Winchester City");
        let club_three = Club::new(3, "Southampton Athletics");
        let club_four = Club::new(4, "Hampshire Rovers");

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

        let clubs = vec![
            club,
            club_two,
            club_three,
            club_four,
        ];

        let league = League::new(
            1,
            "Southern Premier",
            vec![1, 2, 3, 4],
        );

        let fixtures = Self::generate_fixtures(&clubs);

        Self {
            current_day: 1,
            season: 2026,
            running: true,
            clubs,
            league,
            fixtures,
        }
    }

    fn generate_fixtures(clubs: &[Club]) -> Vec<Fixture> {
        let mut fixtures = Vec::new();
        let mut fixture_id = 1;

        for home in clubs {
            for away in clubs {
                if home.id != away.id {
                    fixtures.push(Fixture::new(
                        fixture_id,
                        home.id,
                        away.id,
                    ));

                    fixture_id += 1;
                }
            }
        }

        fixtures
    }

    pub fn advance_day(&mut self) {
        self.current_day += 1;
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}