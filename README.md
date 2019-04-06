# Conway's Game of Life in Rust

Conway's Game of Life in [Rust](https://github.com/rust-lang/rust) using [Piston](https://github.com/PistonDevelopers/piston).

Features in-game cell painting and custom grids from files.

## Screenshot

![screenshot](https://raw.githubusercontent.com/phugh/game-of-life/master/game-of-life.png)

## How to Play

```Mouse click``` to paint cells.

Press ```C``` to clear the grid.

Press ```F``` to display FPS counter.

Press ```L``` to toggle grid lines.

Press ```P``` or to pause.

Press ```R``` to randomise the grid.


## Command Line Flags

The game can be modified by passing command line flags.

### Screen size
```-w``` or ```--width``` to set screen width in pixels. Default is 1280.

```-h``` or ```--height``` to set screen height in pixels. Default is 720.

### Custom grid from image
```-m``` or ```--map``` followed by an image filename, generate grid from image. White pixels (i.e rgba[255, 255, 255, 255]) are dead cells, any other colours will be treated as living cells. Image file must be 1280x720.

For example:
```./game_of_life -m example_map.png```

### Update speed
```-f``` or ```--fps``` followed by a number, controls the number of milliseconds between each grid update.

For example:
```./game_of_life -f 30```

### Cell size
```-s``` or ```--scale``` followed by a number, controls the size of the cells in pixels. Scale must currently be divisible by 1280 and 720.

For example:
```./game_of_life -s 8```

## To Do
- [X] Window size command line flags (inc. start-fullscreen flag)
- [ ] Resize grid on window resize
- [ ] Custom colours
- [X] Toggleable FPS counter on screen
- [ ] Toggleable on-screen game stats
- [ ] Obstacle blocks (maybe?)
- [ ] Refactor main.rs into main.rs and app.rs

## Acknowledgements
Perfect DOS VGA 375 font by Zeh Fernando

## License
Copyright [P. Hughes](https://www.phugh.es) 2019.

This project is licensed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html). See ```LICENSE``` for further details.