use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{self, LeaveAlternateScreen},
    ExecutableCommand,
};
use labrinth::{
    utils::frame::{self, new_frame, Drawable, Frame},
    levels::{level_1::Level1, level_factory::LevelFactory, wall_tile::WallTile, level_2::Level2, door_tile::DoorTile, level_3::Level3},
    entities::{player::Player, enemy::Enemy},
    utils::render::render, items::key::Key,
};
use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in separete thread
    let (render_tx, render_rx) = mpsc::channel::<Frame>();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();

        render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Instantiate game objects
    //let pathfinder = Pathfind::new();
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

    let mut enemy = Enemy::new(0,0);
    if player.current_level == 1 {
        enemy = Enemy::new(level1.enemy.0, level1.enemy.1);
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
                            break 'gameloop;
                        }
                        _ => {}
                    }
                }
            }
        }

        // pathfind test
        // let path = pathfinder.find_path_to((player.x, player.y), (curr_level_keys[0].x, curr_level_keys[0].y), curr_level_tiles);
        // for node in path.iter() {
        //     curr_frame[node.0][node.1] = 'X'.to_string();
        // }

        // Update
        let view_range = player.view_range();
        for key in curr_level_keys.iter() {
            key.detect_player(&mut player);
        }
        if player.current_level == 1 {
            enemy.update(&mut player, delta);
            enemy.draw(&mut curr_frame, &view_range);
        }

        // Draw and render
        let drawables: Vec<&dyn Drawable> = vec![&player, curr_level];
        for drawable in drawables {
            drawable.draw(&mut curr_frame, &view_range);
        }
        let _ = render_tx.send(curr_frame);
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
