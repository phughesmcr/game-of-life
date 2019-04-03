extern crate piston_window;

use piston_window::*;
use std::thread;
use std::time::{Instant, Duration};

mod game;
mod cell;

// Config
const SCALE: usize = 10;
const HEIGHT: usize = 720;
const WIDTH: usize = 1280;
const FRAME_TIME_MS :u64 = 60;

fn main() {
    assert!(WIDTH % SCALE == 0);
    assert!(HEIGHT % SCALE == 0);

    let mut window: PistonWindow = WindowSettings::new("Game of Life", (WIDTH as f64, HEIGHT as f64))
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    
    let mut game = game::Game::new(WIDTH, HEIGHT, SCALE);

    game.init();

    game.randomise();

    game.pause();

    // used later to draw squares to scale
    const S: f64 = SCALE as f64;

    // for grid line drawing
    let mut lines_scale: f64 = SCALE as f64;
    let mut lines: bool = false;

    // mouse coords for mouse painting
    let mut mouse_pos: [f64; 2] = [0.0, 0.0];

    // event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |_c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);

                for cell in game.grid.iter() {
                    let colour = if cell.alive {
                        [0.0, 0.5, 0.0, 1.0]
                    } else {
                        [1.0, 1.0, 1.0, 1.0]
                    };
                    let x = cell.coords[0] as f64;
                    let y = cell.coords[1] as f64;
                    rectangle(colour,
                        rectangle::square(0.0, 0.0, lines_scale),
                        _c.transform.trans(x * S, y * S),
                        g);   
                }
            });
        }

        if let Some(_u) = e.update_args() {
            if !game.paused {
                let last_time = Instant::now();
                // update game state
                game.update();
                // framerate independence
                let delta_time = u64::from((Instant::now() - last_time).subsec_millis());
                if delta_time < FRAME_TIME_MS {
                    thread::sleep(Duration::from_millis(FRAME_TIME_MS - delta_time));
                }
            }
        }

        if let Some(b) = e.release_args() {
            match b {
                Button::Keyboard(key) => {
                    match key {
                        Key::C => {
                            game.init();
                        }
                        Key::R => { 
                            game.randomise();
                        }
                        Key::P => { 
                            game.pause();
                        }
                        Key::L => {
                            if lines {
                                lines_scale = SCALE as f64;
                                lines = false;
                            } else {
                                lines_scale -= 1.0;
                                lines = true;
                            }
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

        if let Some(c) = e.mouse_cursor_args() {
            mouse_pos[0] = c[0];
            mouse_pos[1] = c[1];
        }
    }
}
