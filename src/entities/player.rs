use std::net::TcpStream;

use tokio_tungstenite::tungstenite::{stream::MaybeTlsStream, Message, WebSocket};

use crate::{
    levels::{door_tile::DoorTile, wall_tile::WallTile},
    utils::{
        frame::{Drawable, Frame},
        ws_client::init_server_connection,
    },
    NUM_COLS, NUM_ROWS,
};

pub struct Player {
    pub x: usize,
    pub y: usize,
    hitbox: Vec<(usize, usize, bool)>,
    pub current_level: usize,
    pub keys: Vec<String>,
    pub health: i32,
    pub is_dead: bool,
    pub client: WebSocket<MaybeTlsStream<TcpStream>>,
}

#[allow(unused_assignments)]
impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 5,
            hitbox: vec![(1, 0, true), (1, 0, false), (0, 1, true), (0, 1, false)],
            current_level: 1,
            keys: Vec::new(),
            health: 100,
            is_dead: false,
            client: init_server_connection(),
        }
    }

    pub fn take_hit(&mut self) {
        self.health = self.health - 50;
        if self.health <= 0 {
            self.is_dead = true;
        }
    }

    pub fn move_up(&mut self, level: &Vec<WallTile>, doors: &Vec<DoorTile>) {
        self.detect_doors(doors);
        let can_move = self.detect_walls(level);
        if self.y > 1 && can_move.1 {
            self.y -= 1;
            self.send_location();
        }
    }

    pub fn move_down(&mut self, level: &Vec<WallTile>, doors: &Vec<DoorTile>) {
        self.detect_doors(doors);
        let can_move = self.detect_walls(level);
        if self.y < NUM_ROWS - 2 && can_move.3 {
            self.y += 1;
            self.send_location();
        }
    }

    pub fn move_left(&mut self, level: &Vec<WallTile>, doors: &Vec<DoorTile>) {
        self.detect_doors(doors);
        let can_move = self.detect_walls(level);
        if self.x > 1 && can_move.0 {
            self.x -= 1;
            self.send_location();
        }
    }

    pub fn move_right(&mut self, level: &Vec<WallTile>, doors: &Vec<DoorTile>) {
        self.detect_doors(doors);
        let can_move = self.detect_walls(level);
        if self.x < NUM_COLS - 2 && can_move.2 {
            self.x += 1;
            self.send_location();
        }
    }

    fn send_location(&mut self) {
        self.client
            .send(format!("{},{},{}", self.current_level, self.x, self.y).into())
            .expect("Error on player::send_location");
    }

    pub fn take_key(&mut self, key: String) {
        if !self.keys.contains(&key) {
            self.keys.push(key);
        }
    }

    pub fn view_range(&self) -> Vec<Vec<(usize, usize)>> {
        let mut view_range = Vec::new();
        let mut inner_range = Vec::new();
        let mut outer_range = Vec::new();

        let mut minusx = 0;
        let mut plusx = 0;
        let mut minusy = 0;
        let mut plusy = 0;

        if self.x < 12 {
            minusx = 0;
        } else {
            minusx = self.x - 12;
        }

        if self.x > NUM_COLS - 12 {
            plusx = NUM_COLS;
        } else {
            plusx = self.x + 12;
        }

        if self.y < 6 {
            minusy = 0;
        } else {
            minusy = self.y - 6;
        }

        if self.y > NUM_ROWS - 6 {
            plusy = NUM_ROWS;
        } else {
            plusy = self.y + 6;
        }

        for x in minusx + 2..=plusx - 2 {
            for y in minusy + 2..=plusy - 2 {
                inner_range.push((x, y));
            }
        }

        for x in minusx..=plusx {
            for y in minusy..=plusy {
                outer_range.push((x, y));
            }
        }

        view_range.push(inner_range);
        view_range.push(outer_range);

        view_range
    }

    pub fn detect_walls(&mut self, level: &Vec<WallTile>) -> (bool, bool, bool, bool) {
        let mut can_move = (true, true, true, true);

        for (x, y, positive) in &self.hitbox {
            for wall in level.iter() {
                if *positive {
                    if *x == 1 && *y == 0 {
                        if self.x + *x == wall.x && self.y + *y == wall.y {
                            can_move.2 = false;
                        }
                    } else if *x == 0 && *y == 1 {
                        if self.x + *x == wall.x && self.y + *y == wall.y {
                            can_move.3 = false;
                        }
                    }
                } else {
                    if *x == 1 && *y == 0 {
                        if self.x - *x == wall.x && self.y - *y == wall.y {
                            can_move.0 = false;
                        }
                    } else if *x == 0 && *y == 1 {
                        if self.x - *x == wall.x && self.y - *y == wall.y {
                            can_move.1 = false;
                        }
                    }
                }
            }
        }

        can_move
    }

    pub fn detect_doors(&mut self, doors: &Vec<DoorTile>) {
        for (x, y, positive) in &self.hitbox {
            for door in doors.iter() {
                if door.required_key != None {
                    if !self.keys.contains(&door.required_key.as_ref().unwrap()) {
                        continue;
                    }
                }

                if *positive {
                    if *x == 1 && *y == 0 {
                        if self.x + *x == door.x && self.y + *y == door.y {
                            self.current_level = door.to_level;
                            if door.is_to_side {
                                if door.x <= NUM_COLS / 2 {
                                    self.x = NUM_COLS - 2;
                                } else {
                                    self.x = 1;
                                }
                            } else {
                                if door.y <= NUM_ROWS / 2 {
                                    self.y = NUM_ROWS - 2;
                                } else {
                                    self.y = 1;
                                }
                            }
                        }
                    } else if *x == 0 && *y == 1 {
                        if self.x + *x == door.x && self.y + *y == door.y {
                            self.current_level = door.to_level;
                            if door.is_to_side {
                                if door.x <= NUM_COLS / 2 {
                                    self.x = NUM_COLS - 2;
                                } else {
                                    self.x = 1;
                                }
                            } else {
                                if door.y <= NUM_ROWS / 2 {
                                    self.y = NUM_ROWS - 2;
                                } else {
                                    self.y = 1;
                                }
                            }
                        }
                    }
                } else {
                    if *x == 1 && *y == 0 {
                        if self.x - *x == door.x && self.y - *y == door.y {
                            self.current_level = door.to_level;
                            if door.is_to_side {
                                if door.x <= NUM_COLS / 2 {
                                    self.x = NUM_COLS - 2;
                                } else {
                                    self.x = 1;
                                }
                            } else {
                                if door.y <= NUM_ROWS / 2 {
                                    self.y = NUM_ROWS - 2;
                                } else {
                                    self.y = 1;
                                }
                            }
                        }
                    } else if *x == 0 && *y == 1 {
                        if self.x - *x == door.x && self.y - *y == door.y {
                            self.current_level = door.to_level;
                            if door.is_to_side {
                                if door.x <= NUM_COLS / 2 {
                                    self.x = NUM_COLS - 2;
                                } else {
                                    self.x = 1;
                                }
                            } else {
                                if door.y <= NUM_ROWS / 2 {
                                    self.y = NUM_ROWS - 2;
                                } else {
                                    self.y = 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame, _render_area: &Vec<Vec<(usize, usize)>>) {
        frame[self.x][self.y] = "A".to_string();
        frame[0][0] = format!("Health: {}", self.health.to_string());

        // Draw hitbox
        // for (x, y, positive) in &self.hitbox {
        //     if *positive {
        //         frame[self.x + *x][self.y + *y] = "0".to_string();
        //     }
        //     else {
        //         frame[self.x - *x][self.y - *y] = "0".to_string();
        //     }
        // }
    }
}
