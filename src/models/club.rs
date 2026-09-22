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
}