#[derive(Debug)]
pub struct Fixture {
    pub id: u32,
    pub home_club_id: u32,
    pub away_club_id: u32,
    pub played: bool,
    pub home_goals: Option<u32>,
    pub away_goals: Option<u32>,
}

impl Fixture {
    pub fn new(id: u32, home_club_id: u32, away_club_id: u32) -> Self {
        Self {
            id,
            home_club_id,
            away_club_id,
            played: false,
            home_goals: None,
            away_goals: None,
        }
    }
}