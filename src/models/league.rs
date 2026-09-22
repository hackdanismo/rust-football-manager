#[derive(Debug)]
pub struct LeagueEntry {
    pub club_id: u32,
    pub played: u32,
    pub won: u32,
    pub drawn: u32,
    pub lost: u32,
    pub goals_for: u32,
    pub goals_against: u32,
    pub points: u32,
}

impl LeagueEntry {
    pub fn new(club_id: u32) -> Self {
        Self {
            club_id,
            played: 0,
            won: 0,
            drawn: 0,
            lost: 0,
            goals_for: 0,
            goals_against: 0,
            points: 0,
        }
    }
}

#[derive(Debug)]
pub struct League {
    pub id: u32,
    pub name: String,
    pub table: Vec<LeagueEntry>,
}

impl League {
    pub fn new(
        id: u32,
        name: &str,
        club_ids: Vec<u32>,
    ) -> Self {
        let table = club_ids
            .into_iter()
            .map(LeagueEntry::new)
            .collect();

        Self {
            id,
            name: name.to_string(),
            table,
        }
    }
}