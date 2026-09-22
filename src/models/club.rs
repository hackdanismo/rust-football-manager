use crate::models::player::Player;

#[derive(Debug)]
pub struct Club {
    pub id: u32,
    pub name: String,
    pub players: Vec<Player>,
}

impl Club {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn attacking_strength(&self) -> f32 {
        if self.players.is_empty() {
            return 0.0;
        }

        let total: f32 = self
            .players
            .iter()
            .map(|player| {
                player.shooting as f32 * 0.5
                    + player.passing as f32 * 0.3
                    + player.pace as f32 * 0.2
            })
            .sum();

        total / self.players.len() as f32
    }

    pub fn defensive_strength(&self) -> f32 {
        if self.players.is_empty() {
            return 0.0;
        }

        let total: f32 = self
            .players
            .iter()
            .map(|player| {
                player.tackling as f32 * 0.6
                    + player.pace as f32 * 0.2
                    + player.passing as f32 * 0.2
            })
            .sum();

        total / self.players.len() as f32
    }
}