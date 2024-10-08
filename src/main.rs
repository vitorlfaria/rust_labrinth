use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{self, LeaveAlternateScreen},
    ExecutableCommand,
};
use labrinth::{
    entities::{enemy::Enemy, player::Player},
    items::key::Key,
    levels::{
        door_tile::DoorTile, level_1::Level1, level_2::Level2, level_3::Level3,
        level_factory::LevelFactory, wall_tile::WallTile,
    },
    screens::lose_screen::generate_lose_screen,
    utils::frame::{self, new_frame, Drawable, Frame},
    utils::render::render,
};
use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in separete thread
    let (render_tx, render_rx) = mpsc::channel::<(Frame, bool)>();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();

        render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let (curr_frame, force) = match render_rx.recv() {
                Ok((x, y)) => (x, y),
                Err(_) => break,
            };
            render(&mut stdout, &last_frame, &curr_frame, force);
            last_frame = curr_frame;
        }
    });

    // Instantiate game objects
    let mut instant = Instant::now();
    let mut player = Player::new();
    let mut levels: Vec<&dyn Drawable> = Vec::new();
    let mut levels_tiles: Vec<&Vec<WallTile>> = Vec::new();
    let mut levels_doors: Vec<&Vec<DoorTile>> = Vec::new();
    let mut levels_keys: Vec<&Vec<Key>> = Vec::new();

    // Instatiate levels_tiles
    let mut level1 = Level1::new();
    let mut level2 = Level2::new();
    let mut level3 = Level3::new();

    level1.create_level();
    level2.create_level();
    level3.create_level();

    levels.push(&level1);
    levels.push(&level2);
    levels.push(&level3);

    // Take levels tiles
    levels_tiles.push(&level1.tiles);
    levels_tiles.push(&level2.tiles);
    levels_tiles.push(&level3.tiles);

    // Take levels doors
    levels_doors.push(&level1.doors);
    levels_doors.push(&level2.doors);
    levels_doors.push(&level3.doors);

    // Take levels keys
    levels_keys.push(&level1.keys);
    levels_keys.push(&level2.keys);
    levels_keys.push(&level3.keys);

    let mut enemy = Enemy::new(0, 0, vec![]);
    if player.current_level == 1 {
        enemy = Enemy::new(level1.enemy.0, level1.enemy.1, level1.patrol_points.clone());
    }

    // Game loop
    'gameloop: loop {
        // Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();
        let curr_level = levels[player.current_level - 1];
        let curr_level_tiles = levels_tiles[player.current_level - 1];
        let curr_level_doors = levels_doors[player.current_level - 1];
        let curr_level_keys = levels_keys[player.current_level - 1];

        // Input
        while event::poll(Duration::default()).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('w') => player.move_up(curr_level_tiles, curr_level_doors),
                        KeyCode::Char('s') => player.move_down(curr_level_tiles, curr_level_doors),
                        KeyCode::Char('a') => player.move_left(curr_level_tiles, curr_level_doors),
                        KeyCode::Char('d') => player.move_right(curr_level_tiles, curr_level_doors),
                        KeyCode::Esc => {
                            player
                                .client
                                .close(None)
                                .expect("Error while closing server connection");
                            break 'gameloop;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Update
        let view_range = player.view_range();
        for key in curr_level_keys.iter() {
            key.detect_player(&mut player);
        }
        if player.current_level == 1 {
            enemy.update(&mut player, delta, &curr_level_tiles);
            enemy.draw(&mut curr_frame, &view_range);
        }

        // Draw and render
        let drawables: Vec<&dyn Drawable> = vec![&player, curr_level];
        for drawable in drawables {
            drawable.draw(&mut curr_frame, &view_range);
        }
        let _ = render_tx.send((curr_frame, false));

        // Win or Lose
        if player.is_dead {
            let lose_screen = generate_lose_screen();
            let _ = render_tx.send((lose_screen, true));
            thread::sleep(Duration::from_millis(4000));
            break 'gameloop;
        }

        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
