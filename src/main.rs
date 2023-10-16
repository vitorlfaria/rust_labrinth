use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{self, LeaveAlternateScreen},
    ExecutableCommand,
};
use labrinth::{frame::{new_frame, Frame, self, Drawable}, render::render, player::Player, levels::{self, level_1}};
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

    // Game loop
    let mut instant = Instant::now();
    let mut player = Player::new();
    let mut levels = instantiate_levels();

    'gameloop: loop {
        // Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // Input
        while event::poll(Duration::default()).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Left => player.move_left(),
                        KeyCode::Right => player.move_right(),
                        KeyCode::Up => player.move_up(),
                        KeyCode::Down => player.move_down(),
                        KeyCode::Esc => {
                            break 'gameloop;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Update
        player.update();

        // Draw and render
        let mut drawables: Vec<&dyn Drawable> = vec![&player];
        for level in &levels {
            drawables.push(level);
        }
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

pub fn instantiate_levels() -> Vec<level_1::Level> {
    let mut levels = Vec::new();
    levels.push(level_1::Level::new());
    levels
}
