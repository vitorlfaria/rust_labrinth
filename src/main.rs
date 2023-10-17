use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{self, LeaveAlternateScreen},
    ExecutableCommand,
};
use labrinth::{frame::{new_frame, Frame, self, Drawable}, render::render, player::Player, levels::{level_1::Level1, level_factory::LevelFactory, wall_tile::WallTile}};
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
    let mut instant = Instant::now();
    let mut player = Player::new();
    let mut levels: Vec<&Vec<WallTile>> = vec![];

    // Instatiate levels
    let mut level1 = Level1::new();
    
    level1.create_level();

    levels.push(&level1.tiles);
    
    // Game loop
    'gameloop: loop {
        // Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();
        let curr_level = levels[player.current_level - 1];

        // Input
        while event::poll(Duration::default()).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('w') => player.move_up(curr_level),
                        KeyCode::Char('s') => player.move_down(curr_level),
                        KeyCode::Char('a') => player.move_left(curr_level),
                        KeyCode::Char('d') => player.move_right(curr_level),
                        KeyCode::Esc => {
                            break 'gameloop;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Update

        // Draw and render
        let drawables: Vec<&dyn Drawable> = vec![&player, &level1];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
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
