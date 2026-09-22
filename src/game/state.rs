use rand::random_range;

use crate::models::{
    club::Club,
    fixture::Fixture,
    league::League,
    player::{Player, Position},
};

#[derive(Debug)]
pub struct GameState {
    pub current_day: u32,
    pub current_matchday: u32,
    pub season: u32,
    pub running: bool,
    pub fixtures: Vec<Fixture>,
    pub league: League,
    pub clubs: Vec<Club>,
}

impl GameState {
    pub fn new() -> Self {
        let mut club = Club::new(1, "Eastleigh Town");
        let mut club_two = Club::new(2, "Winchester City");
        let mut club_three = Club::new(3, "Southampton Athletics");
        let mut club_four = Club::new(4, "Hampshire Rovers");

        // Eastleigh Town

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

        // Winchester City

        club_two.add_player(Player::new(
            5,
            "Daniel Foster",
            30,
            Position::Goalkeeper,
            38,
            10,
            41,
            46,
        ));

        club_two.add_player(Player::new(
            6,
            "Luke Bennett",
            26,
            Position::Defender,
            52,
            27,
            76,
            61,
        ));

        club_two.add_player(Player::new(
            7,
            "Oliver Price",
            23,
            Position::Midfielder,
            69,
            58,
            57,
            65,
        ));

        club_two.add_player(Player::new(
            8,
            "Charlie Adams",
            28,
            Position::Forward,
            51,
            73,
            31,
            68,
        ));

        // Southampton Athletics

        club_three.add_player(Player::new(
            9,
            "Nathan Green",
            29,
            Position::Goalkeeper,
            45,
            9,
            38,
            48,
        ));

        club_three.add_player(Player::new(
            10,
            "Adam Clarke",
            25,
            Position::Defender,
            61,
            34,
            69,
            72,
        ));

        club_three.add_player(Player::new(
            11,
            "George Wilson",
            21,
            Position::Midfielder,
            78,
            65,
            49,
            77,
        ));

        club_three.add_player(Player::new(
            12,
            "Harry Collins",
            24,
            Position::Forward,
            59,
            82,
            25,
            80,
        ));

        // Hampshire Rovers

        club_four.add_player(Player::new(
            13,
            "Matthew Hill",
            31,
            Position::Goalkeeper,
            40,
            8,
            42,
            44,
        ));

        club_four.add_player(Player::new(
            14,
            "Samuel Cooper",
            27,
            Position::Defender,
            49,
            25,
            66,
            58,
        ));

        club_four.add_player(Player::new(
            15,
            "Thomas Ward",
            26,
            Position::Midfielder,
            65,
            55,
            54,
            62,
        ));

        club_four.add_player(Player::new(
            16,
            "Joseph Bailey",
            29,
            Position::Forward,
            48,
            68,
            29,
            64,
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
            current_matchday: 1,
            season: 2026,
            running: true,
            clubs,
            league,
            fixtures,
        }
    }

    fn generate_fixtures(clubs: &[Club]) -> Vec<Fixture> {
        let mut fixtures = Vec::new();

        if clubs.len() < 2 {
            return fixtures;
        }

        assert!(
            clubs.len().is_multiple_of(2),
            "Fixture generator currently requires an even number of clubs"
        );

        let mut team_ids: Vec<u32> =
            clubs.iter().map(|club| club.id).collect();

        let team_count = team_ids.len();
        let rounds = team_count - 1;
        let matches_per_round = team_count / 2;

        let mut fixture_id = 1;

        // First half of the season.
        for round in 0..rounds {
            let matchday = round as u32 + 1;

            for index in 0..matches_per_round {
                let first = team_ids[index];
                let second = team_ids[team_count - 1 - index];

                let (home_club_id, away_club_id) =
                    if index == 0 {
                        if round % 2 == 0 {
                            (first, second)
                        } else {
                            (second, first)
                        }
                    } else if round % 2 == 0 {
                        (second, first)
                    } else {
                        (first, second)
                    };

                fixtures.push(Fixture::new(
                    fixture_id,
                    matchday,
                    home_club_id,
                    away_club_id,
                ));

                fixture_id += 1;
            }

            // Keep the first club fixed and rotate the others.
            let fixed_team = team_ids[0];

            let mut rotating_teams =
                team_ids[1..].to_vec();

            rotating_teams.rotate_right(1);

            team_ids.clear();
            team_ids.push(fixed_team);
            team_ids.extend(rotating_teams);
        }

        // Second half of the season.
        // Reverse every first-leg fixture so home becomes away.
        let first_leg_fixture_count = fixtures.len();

        for index in 0..first_leg_fixture_count {
            let first_leg = &fixtures[index];

            let matchday =
                first_leg.matchday + rounds as u32;

            let home_club_id =
                first_leg.away_club_id;

            let away_club_id =
                first_leg.home_club_id;

            fixtures.push(Fixture::new(
                fixture_id,
                matchday,
                home_club_id,
                away_club_id,
            ));

            fixture_id += 1;
        }

        fixtures
    }

    pub fn simulate_current_matchday(&mut self) {
        let fixture_indices: Vec<usize> = self
            .fixtures
            .iter()
            .enumerate()
            .filter(|(_, fixture)| {
                fixture.matchday == self.current_matchday
                    && !fixture.played
            })
            .map(|(index, _)| index)
            .collect();

        if fixture_indices.is_empty() {
            println!("No fixtures left to play.");
            return;
        }

        println!();
        println!("Matchday {}", self.current_matchday);
        println!("==============================================");

        for fixture_index in fixture_indices {
            let home_club_id =
                self.fixtures[fixture_index].home_club_id;

            let away_club_id =
                self.fixtures[fixture_index].away_club_id;

            let (
                home_name,
                home_attack,
                home_defence,
            ) = {
                let home_club = self
                    .clubs
                    .iter()
                    .find(|club| club.id == home_club_id)
                    .expect("Home club not found");

                (
                    home_club.name.clone(),
                    home_club.attacking_strength(),
                    home_club.defensive_strength(),
                )
            };

            let (
                away_name,
                away_attack,
                away_defence,
            ) = {
                let away_club = self
                    .clubs
                    .iter()
                    .find(|club| club.id == away_club_id)
                    .expect("Away club not found");

                (
                    away_club.name.clone(),
                    away_club.attacking_strength(),
                    away_club.defensive_strength(),
                )
            };

            let home_goals = Self::simulate_goals(
                home_attack,
                away_defence,
                true,
            );

            let away_goals = Self::simulate_goals(
                away_attack,
                home_defence,
                false,
            );

            {
                let fixture =
                    &mut self.fixtures[fixture_index];

                fixture.home_goals = Some(home_goals);
                fixture.away_goals = Some(away_goals);
                fixture.played = true;
            }

            self.update_league_table(
                home_club_id,
                away_club_id,
                home_goals,
                away_goals,
            );

            println!(
                "{:<24} {} - {} {}",
                home_name,
                home_goals,
                away_goals,
                away_name,
            );
        }

        self.current_matchday += 1;
    }

    fn simulate_goals(
        attacking_strength: f32,
        defensive_strength: f32,
        home_advantage: bool,
    ) -> u32 {
        let mut chance =
            18.0 + (attacking_strength - defensive_strength) * 0.4;

        if home_advantage {
            chance += 4.0;
        }

        chance = chance.clamp(7.0, 40.0);

        let mut goals = 0;

        for _ in 0..8 {
            let roll: f32 =
                random_range(0.0..100.0);

            if roll < chance {
                goals += 1;
            }
        }

        goals
    }

    fn update_league_table(
        &mut self,
        home_club_id: u32,
        away_club_id: u32,
        home_goals: u32,
        away_goals: u32,
    ) {
        let home_index = self
            .league
            .table
            .iter()
            .position(|entry| {
                entry.club_id == home_club_id
            })
            .expect(
                "Home club not found in league table",
            );

        let away_index = self
            .league
            .table
            .iter()
            .position(|entry| {
                entry.club_id == away_club_id
            })
            .expect(
                "Away club not found in league table",
            );

        self.league.table[home_index].played += 1;
        self.league.table[away_index].played += 1;

        self.league.table[home_index].goals_for +=
            home_goals;

        self.league.table[home_index].goals_against +=
            away_goals;

        self.league.table[away_index].goals_for +=
            away_goals;

        self.league.table[away_index].goals_against +=
            home_goals;

        if home_goals > away_goals {
            self.league.table[home_index].won += 1;
            self.league.table[away_index].lost += 1;

            self.league.table[home_index].points += 3;
        } else if away_goals > home_goals {
            self.league.table[away_index].won += 1;
            self.league.table[home_index].lost += 1;

            self.league.table[away_index].points += 3;
        } else {
            self.league.table[home_index].drawn += 1;
            self.league.table[away_index].drawn += 1;

            self.league.table[home_index].points += 1;
            self.league.table[away_index].points += 1;
        }

        self.sort_league_table();
    }

    fn sort_league_table(&mut self) {
        self.league.table.sort_by(|a, b| {
            let a_goal_difference =
                a.goals_for as i32
                    - a.goals_against as i32;

            let b_goal_difference =
                b.goals_for as i32
                    - b.goals_against as i32;

            b.points
                .cmp(&a.points)
                .then(
                    b_goal_difference
                        .cmp(&a_goal_difference),
                )
                .then(
                    b.goals_for
                        .cmp(&a.goals_for),
                )
        });
    }

    pub fn advance_day(&mut self) {
        self.current_day += 1;
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}