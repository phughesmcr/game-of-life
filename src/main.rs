use clap::Arg;
use fps_counter::FPSCounter;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston_window::{clear, rectangle, text, Button, EventSettings, Events, Key, MouseCursorEvent, PistonWindow, PressEvent, RenderEvent, Transformed, UpdateEvent, WindowSettings};
use std::thread;
use std::time::{Duration};

mod cell;
mod game;

// RGBA colours
const ALIVE: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
const DEAD: [f32; 4]= [0.9; 4];

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
                .arg(Arg::with_name("width")
                    .short("w")
                    .long("width")
                    .value_name("WIDTH")
                    .help("Sets the screen width in px.")
                    .takes_value(true)
                    .validator(is_positive))
                .arg(Arg::with_name("height")
                    .short("h")
                    .long("height")
                    .value_name("HEIGHT")
                    .help("Sets the screen height in px.")
                    .takes_value(true)
                    .validator(is_positive))
                .arg(Arg::with_name("fullscreen")
                    .short("x")
                    .long("fullscreen")
                    .help("Run the game in full screen mode."))
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

    let width: f64 = matches.value_of("width").map(|valstr| valstr.parse::<f64>().unwrap())
                                        .unwrap_or(1280.0);
    let height: f64 = matches.value_of("height").map(|valstr| valstr.parse::<f64>().unwrap())
                                        .unwrap_or(720.0);
    let frame_time_ms: f64 = matches.value_of("fps").map(|valstr| valstr.parse::<f64>().unwrap())
                                        .unwrap_or(15.0);
    let scale: f64 = matches.value_of("scale").map(|valstr| valstr.parse::<f64>().unwrap())
                                        .unwrap_or(8.0);
    let map_file = matches.value_of("map").unwrap_or("default");


    assert!(width % scale == 0.0);
    assert!(height % scale == 0.0);

    // window setup
    let opengl = OpenGL::V3_2;

    let fullscreen = matches.occurrences_of("fullscreen") > 0;

    let mut window: PistonWindow = WindowSettings::new("Game of Life", (width, height))
        .opengl(opengl)
        .vsync(true)
        .fullscreen(fullscreen)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .expect("Failed to build window!");
    
    let mut gl = GlGraphics::new(opengl);

    gl.viewport(0, 0, width as i32, height as i32);
    
    // game setup
    let mut game = game::Game::new(width, height, scale);

    game.init();
    
    if map_file == "default" {
        game.randomise();
    } else {
        game.image_to_grid(map_file);
    }

    game.toggle_pause();

    // for grid line drawing
    let mut lines: bool = false;
    let mut lines_scale: f64 = scale;

    // mouse coords for mouse painting
    let mut mouse_pos: [f64; 2] = [0.0, 0.0];

    // fps counter
    let mut fps_display: bool = false;
    let mut fps = FPSCounter::new();
    let fps_str: &str = " fps";

    // fonts for screen text
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("res").unwrap();
    let font = &assets.join("Perfect-DOS-VGA-437-Win.ttf");
    let mut glyphs = GlyphCache::new(font, (), TextureSettings::new()).expect("Unable to load font");

    // event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            if !game.paused {
                // main draw call
                gl.draw(r.viewport(), |c, g| {
                    // clear screen
                    clear([0.5; 4], g);
                    // draw cells
                    for cell in game.grid.iter() {
                        let colour = if cell.alive {
                            ALIVE
                        } else {
                            DEAD
                        };
                        rectangle(colour,
                            rectangle::square(0.0, 0.0, lines_scale),
                            c.transform.trans(
                                cell.coords[0] * scale, 
                                cell.coords[1] * scale),
                            g);   
                    }
                    // FPS counter
                    if fps_display {
                        let mut t = fps.tick().to_string();
                        t.push_str(&fps_str);
                        let fps_tick: &str = &t;

                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 15).draw(
                            fps_tick,
                            &mut glyphs,
                            &c.draw_state,
                            c.transform.trans(5.0, 15.0), 
                        g).unwrap();
                    }
                });
            }
        }
        
        if let Some(u) = e.update_args() {
            // update game state
            if !game.paused {
                game.update();
            }
            // framerate independence
            if u.dt < frame_time_ms {
                thread::sleep(Duration::from_millis((frame_time_ms - u.dt) as u64));
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
                        Key::F => {
                            fps_display = !fps_display;
                        }
                        Key::R => {
                            game.randomise();
                        }
                        Key::P => { 
                            game.toggle_pause();
                        }
                        Key::L => {
                            // toggle grid lines
                            lines = !lines;
                            lines_scale = if lines {
                                scale - 1.0
                            } else {
                                scale
                            };
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
