use piston_window::{clear, rectangle, Button, EventSettings, Events, Key, MouseCursorEvent, PistonWindow, PressEvent, RenderEvent, Transformed, UpdateEvent, WindowSettings};
use std::thread;
use std::time::{Instant, Duration};

mod cell;
mod game;

// Config
const SCALE: usize = 10;
const HEIGHT: usize = 720;
const WIDTH: usize = 1280;
const FRAME_TIME_MS: u64 = 60;
// RGBA colours
const ALIVE: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
const DEAD: [f32; 4]= [1.0, 1.0, 1.0, 1.0];

fn main() {
    assert!(WIDTH % SCALE == 0);
    assert!(HEIGHT % SCALE == 0);

    let mut window: PistonWindow = WindowSettings::new("Game of Life", (WIDTH as f64, HEIGHT as f64))
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .expect("Failed to build window!");
    
    let mut game = game::Game::new(WIDTH, HEIGHT, SCALE);

    game.init();

    game.randomise();

    game.toggle_pause();

    // used later to draw squares to scale
    const S: f64 = SCALE as f64;

    // for grid line drawing
    let mut lines: bool = false;

    // mouse coords for mouse painting
    let mut mouse_pos: [f64; 2] = [0.0, 0.0];

    // event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if e.render_args().is_some() {
            // check if we want grid lines
            let lines_scale = if lines {
                S - 1.0
            } else {
                S
            };
            // DRAW!
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g); // clear screen

                for cell in game.grid.iter() {
                    let colour = if cell.alive {
                        ALIVE
                    } else {
                        DEAD
                    };
                    rectangle(colour,
                        rectangle::square(0.0, 0.0, lines_scale),
                        c.transform.trans(
                            cell.coords[0] as f64 * S, 
                            cell.coords[1] as f64 * S),
                        g);   
                }
            });
        }
        
        if e.update_args().is_some() && !game.paused {
            let last_time = Instant::now();
            // update game state
            game.update();
            // framerate independence
            let delta_time = u64::from((Instant::now() - last_time).subsec_millis());
            if delta_time < FRAME_TIME_MS {
                thread::sleep(Duration::from_millis(FRAME_TIME_MS - delta_time));
            }
        }
        
        if let Some(b) = e.press_args() {
            match b {
                Button::Keyboard(key) => {
                    match key {
                        Key::C => {
                            // clear the grid
                            game.init();
                        }
                        Key::R => {
                            game.randomise();
                        }
                        Key::P => { 
                            game.toggle_pause();
                        }
                        Key::L => {
                            lines = !lines;
                        }
                        _ => {}
                    }
                },
                Button::Mouse(_button) => {
                    game.paint(mouse_pos);
                },
                _ => { }
            }
        }

        if let Some(new_pos) = e.mouse_cursor_args() {
            mouse_pos = new_pos;
        }
    }
}
