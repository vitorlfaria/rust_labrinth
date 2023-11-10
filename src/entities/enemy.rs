use std::time::Duration;
use crossterm::style::Stylize;
use rusty_time::Timer;
use crate::{utils::{frame::{Drawable, Frame}, pathfind_trait::Pathfind}, levels::wall_tile::WallTile};
use super::player::Player;

pub struct Enemy {
    x: usize,
    y: usize,
    idle_timer: Timer,
    chase_timer: Timer,
    patrol_timer: Timer,
    current_state: &'static str,
    route: Vec<(usize, usize)>,
    graphic: char,
    hitbox: Vec<(usize, usize, bool)>,
    pathfinder: Pathfind,
}

impl Enemy {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            idle_timer: Timer::from_millis(2000),
            chase_timer: Timer::from_millis(200),
            patrol_timer: Timer::from_millis(500),
            current_state: "idle",
            route: vec![(0,0)],
            graphic: 'E',
            hitbox: vec![(1, 0, true), (1, 0, false), (0, 1, true), (0, 1, false)],
            pathfinder: Pathfind::new(),
        }
    }

    pub fn update(&mut self, player: &Player, delta: Duration, level: &Vec<WallTile>) {
        self.detect_player(player, delta, level);

        if self.current_state == "idle" {
            if self.idle_timer.ready {
                self.idle_timer.update(delta);
                self.current_state = "patrol";
                self.idle_timer.reset();
            }
        }
    }

    pub fn detect_player(&mut self, player: &Player, delta: Duration, level: &Vec<WallTile>) {
        let range = player.view_range();
        if range[0].contains(&(self.x, self.y)) {
            self.current_state = "chase";
            self.chase_logic(player, delta, level);
        }
    }

    fn chase_logic(&mut self, player: &Player, delta: Duration, level: &Vec<WallTile>) {
        self.chase_timer.update(delta);
        
        if self.chase_timer.ready {
            let path = self.pathfinder.find_path_to((self.x, self. y), (player.x, player.y), level);
            if path.len() > 0 {
                self.x = path[0].0;
                self.y = path[0].1;
            }

            self.chase_timer.reset();
        }
    }
}

impl Drawable for Enemy {
    fn draw(&self, frame: &mut Frame, _render_area: &Vec<Vec<(usize, usize)>>) {
        if _render_area[1].contains(&(self.x, self.y)) {
            frame[self.x][self.y] = self.graphic.dark_grey().to_string();
        }
        if _render_area[0].contains(&(self.x, self.y)) {
            frame[self.x][self.y] = self.graphic.to_string();
        }
    }
}