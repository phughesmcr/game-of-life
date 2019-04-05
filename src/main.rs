use clap::Arg;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston_window::{clear, rectangle, Button, EventSettings, Events, Key, MouseCursorEvent, PistonWindow, PressEvent, RenderEvent, Transformed, UpdateEvent, WindowSettings};
use std::thread;
use std::time::{Instant, Duration};

mod cell;
mod game;

// Config
const HEIGHT: usize = 720;
const WIDTH: usize = 1280;
// RGBA colours
const ALIVE: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
const DEAD: [f32; 4]= [1.0, 1.0, 1.0, 1.0];

fn is_positive(valstr: String) -> Result<(), String> {
    if valstr.parse::<u32>().is_ok() {
        Ok(())
    } else {
        Err(format!("{} is not a valid number (must be positive)", valstr))
    }
}

fn main() {
    // parse flags
    let matches = clap::App::new("game-of-life")
                .arg(Arg::with_name("fps")
                    .short("f")
                    .long("fps")
                    .value_name("FPS")
                    .help("Sets the ms between each update")
                    .takes_value(true)
                    .validator(is_positive))
                .arg(Arg::with_name("scale")
                    .short("s")
                    .long("scale")
                    .value_name("SCALE")
                    .help("Sets the size of the cells")
                    .takes_value(true)
                    .validator(is_positive))
                .arg(Arg::with_name("map")
                    .short("m")
                    .long("map")
                    .value_name("FILE")
                    .help("Game grid from image")
                    .takes_value(true))
                .get_matches();


    let frame_time_ms = matches.value_of("fps").map(|valstr| valstr.parse::<u64>().unwrap())
                                        .unwrap_or(30);
    let scale: usize = matches.value_of("scale").map(|valstr| valstr.parse::<u32>().unwrap())
                                        .unwrap_or(5) as usize;
    let map_file = matches.value_of("map").unwrap_or("default");

    assert!(WIDTH % scale == 0);
    assert!(HEIGHT % scale == 0);

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("Game of Life", (WIDTH as f64, HEIGHT as f64))
        .opengl(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .expect("Failed to build window!");

    let mut gl = GlGraphics::new(opengl);
    
    let mut game = game::Game::new(WIDTH, HEIGHT, scale);

    game.init();
    
    if map_file == "default" {
        game.randomise();
    } else {
        game.image_to_grid(map_file);
    }

    game.toggle_pause();

    // used later to draw squares to scale
    let scale_f64: f64 = scale as f64;

    // for grid line drawing
    let mut lines: bool = false;

    // mouse coords for mouse painting
    let mut mouse_pos: [f64; 2] = [0.0, 0.0];

    // event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            // check if we want grid lines
            let lines_scale = if lines {
                scale_f64 - 1.0
            } else {
                scale_f64
            };
            // DRAW!
            gl.viewport(0, 0, r.width as i32, r.height as i32);

            gl.draw(r.viewport(), |c, g| {
                clear([1.0, 1.0, 1.0, 1.0], g);

                for cell in game.grid.iter() {
                    let colour = if cell.alive {
                        ALIVE
                    } else {
                        DEAD
                    };
                    rectangle(colour,
                        rectangle::square(0.0, 0.0, lines_scale),
                        c.transform.trans(
                            cell.coords[0] as f64 * scale_f64, 
                            cell.coords[1] as f64 * scale_f64),
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
            if delta_time < frame_time_ms {
                thread::sleep(Duration::from_millis(frame_time_ms - delta_time));
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
