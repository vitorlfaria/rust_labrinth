pub struct Enemy {
    x: usize,
    y: usize,
}

impl Enemy {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
}