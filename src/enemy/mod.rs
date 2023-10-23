pub mod EnemyStateManager;
pub mod EnemyPatrolState;

pub struct Enemy {
    pub x: usize,
    pub y: usize,
    hitbox: Vec<(usize, usize, bool)>,
}

impl Enemy {
    pub fn new(&self, x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            hitbox: vec![(1, 0, true), (1, 0, false), (0, 1, true), (0, 1, false)],
        }
    }
}