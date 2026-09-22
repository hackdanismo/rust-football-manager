#[derive(Debug)]
pub enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub position: Position,
    pub passing: u8,
    pub shooting: u8,
    pub tackling: u8,
    pub pace: u8,
}

impl Player {
    pub fn new(
        id: u32,
        name: &str,
        age: u8,
        position: Position,
        passing: u8,
        shooting: u8,
        tackling: u8,
        pace: u8,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            age,
            position,
            passing,
            shooting,
            tackling,
            pace,
        }
    }
}