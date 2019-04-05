#[derive(Clone, Copy)]
pub struct Cell {
  pub alive: bool,
  pub coords: [usize; 2],
  pub neighbours: [usize; 8]
}

impl Cell {
  pub fn new() -> Cell {
    Cell {
      alive: false,
      coords: [0, 0],
      neighbours: [0, 0, 0, 0, 0, 0, 0, 0]
    }
  }

  pub fn init(&mut self, cell: usize, width: usize, height: usize) {
    self.alive = false;
    self.coords = self.get_coords(cell, width);
    self.neighbours = self.get_neighbours(cell, width, height);
    // println!("Cell: {}, coords: {:?}, neighbours: {:?}", cell, self.coords, self.neighbours);
  }

  pub fn toggle_life(&mut self) {
    self.alive = !self.alive;
  }

  pub fn get_coords(&self, n: usize, w: usize) -> [usize; 2] {
    let x = n % w;
    let y = (n / w) as f32;
    [x, y.floor() as usize] // (col, row)
  }

  pub fn get_neighbours(&self, cell: usize, width: usize, height: usize) -> [usize; 8] {
    // Total cells
    let size = width * height;

    // Setup
    let lc = ((((cell / width) * width) as f64).floor()) as usize;    // left most cell
    let rc = (lc + width) - 1;                                        // right most cell
    let size_minus_width = size - width;
    let cell_minus_width = (cell - width) as i32;
    let cell_plus_width = cell + width;
    let cell_mod_width = cell % width;
    let top_right = width - 1;
    let bottom_right = size - 1;

    // Directions
    let north;
    let south;
    let east; 
    let west;
    let north_east;
    let north_west;
    let south_east;
    let south_west;

    // North
    if cell_minus_width < 0 {                   // TOP EDGE
      north = size_minus_width + cell;
    } else {
      north = cell_minus_width as usize;
    }

    // South
    if cell_plus_width >= size {                // BOTTOM EDGE
      south = cell - lc;
    } else {
      south = cell_plus_width;
    }

    // East, North-East, South-East
    if cell_mod_width == top_right {           // RIGHT EDGE
      east = lc;
      if cell == top_right {                   // top right corner
        north_east = size_minus_width;
        south_east = lc + width;
      } else if cell == bottom_right {         // bottom right corner
        north_east = lc - width;
        south_east = 0;
      } else {
        north_east = lc - width;
        south_east = lc + width;
      }
    } else {
      east = cell + 1;
      north_east = north + 1;
      south_east = south + 1;
    }

    // West, North-West, South-West
    if cell_mod_width == 0 {                   // LEFT EDGE
      west = rc;
      if cell == size_minus_width {            // bottom left corner
        north_west = cell - 1;
        south_west = top_right;
      } else if cell == 0 {                    // top left corner
        north_west = bottom_right;
        south_west = rc + width;
      } else {
        north_west = cell - 1;
        south_west = rc + width;
      }
    } else {
      west = cell - 1;
      north_west = north - 1;
      south_west = south - 1;
    }

    [north, south, east, west, north_west, north_east, south_west, south_east]
  }
}
