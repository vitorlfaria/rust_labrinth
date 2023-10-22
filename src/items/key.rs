use crate::player::Player;

pub struct Key {
    pub x: usize,
    pub y: usize,
    pub name: String,
}

impl Key {
    pub fn detect_player(&self, player: &mut Player) {
        if self.x == player.x && self.y == player.y {
            player.take_key(self.name.clone());
        }
    }
}