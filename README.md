# Conway's Game of Life in Rust

Conway's Game of Life in [Rust](https://github.com/rust-lang/rust) using [Piston](https://github.com/PistonDevelopers/piston).

Features in-game cell painting and custom grids from files.

## Screenshot

![screenshot](https://raw.githubusercontent.com/phugh/game-of-life/master/game-of-life.png)

## How to Play

```Mouse click``` to paint cells.

Press ```C``` to clear the grid.

Press ```R``` to randomise the grid.

Press ```P``` or to pause.

Press ```L``` to toggle grid lines.

## Command Line Flags

The game can be modified by passing command line flags.

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
- [ ] Window size command line flags (inc. start-fullscreen flag)
- [ ] Resize grid on window resize
- [ ] Custom colours
- [ ] Toggleable FPS counter on screen
- [ ] Toggleable on-screen game stats
- [ ] Obstacle blocks (maybe?)

## License
Copyright [P. Hughes](https://www.phugh.es) 2019.

This project is licensed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html). See ```LICENSE``` for further details.