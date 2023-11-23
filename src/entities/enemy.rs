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
    hit_timer: Timer,
    current_state: &'static str,
    patrol_points: Vec<(usize, usize)>,
    current_point: usize,
    route: Vec<(usize, usize, usize)>,
    graphic: char,
    pathfinder: Pathfind,
}

impl Enemy {
    pub fn new(x: usize, y: usize, patrol_points: Vec<(usize, usize)>) -> Self {
        Self {
            x,
            y,
            idle_timer: Timer::from_millis(2000),
            chase_timer: Timer::from_millis(200),
            patrol_timer: Timer::from_millis(500),
            hit_timer: Timer::from_millis(100),
            current_state: "idle",
            patrol_points,
            route: vec![],
            current_point: 0,
            graphic: '§',
            pathfinder: Pathfind::new(),
        }
    }

    pub fn update(&mut self, player: &mut Player, delta: Duration, level: &Vec<WallTile>) {
        let player_in_range: bool = self.detect_player_range(player);
        
        if player_in_range {
            self.current_state = "chase";
        }
        else if self.current_state == "chase" {
            self.current_state = "idle";
        }

        match self.current_state {
            "idle" => {
                self.idle_logic(delta);
            },
            "chase" => {
                self.chase_logic(player, delta, level);
            },
            "patrol" => {
                self.patrol_logic(delta, level);
            },
            _ => {}
        }
    }

    pub fn detect_player_range(&mut self, player: &Player) -> bool {
        let range = player.view_range();
        if range[0].contains(&(self.x, self.y)) {
            return true;
        }
        false
    }

    fn patrol_logic(&mut self, delta: Duration, level: &Vec<WallTile>) {
        self.current_point += 1;
        if self.current_point > self.patrol_points.len() - 1 {
            self.current_point = 0;
        }

        let next_point = self.patrol_points[self.current_point];
        if self.route.len() == 0 {
            let mut route = self.pathfinder.find_path_to((self.x, self.y), next_point, level);
            route.reverse();
            self.route = route;
        }

        self.patrol_timer.update(delta);
        if self.patrol_timer.ready {
            let walk_to = self.route.pop().unwrap(); 
            self.x = walk_to.0;
            self.y = walk_to.1;
            self.patrol_timer.reset();
        }
    }

    fn idle_logic(&mut self, delta: Duration) {
        self.idle_timer.update(delta);
        if self.idle_timer.ready {
            self.current_state = "patrol";
            self.route = vec![];
            self.idle_timer.reset();
        }
    }

    fn chase_logic(&mut self, player: &mut Player, delta: Duration, level: &Vec<WallTile>) {
        self.chase_timer.update(delta);
        
        if self.chase_timer.ready {
            let path = self.pathfinder.find_path_to((self.x, self. y), (player.x, player.y), level);
            if path.len() > 0 {
                self.x = path[0].0;
                self.y = path[0].1;
            }
            
            if path.len() <= 2 {
                self.hit_player(player, delta);
            }

            self.chase_timer.reset();
        }
    }
    
    fn hit_player(&mut self, player: &mut Player, delta: Duration) {
        self.hit_timer.update(delta);

        if self.hit_timer.ready {
            player.take_hit();
            self.hit_timer.reset();
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
