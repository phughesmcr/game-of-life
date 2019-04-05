use rand::Rng;
use crate::cell::Cell;

pub struct Game {
  pub width: usize,
  pub height: usize,
  pub scale: usize,
  pub size: usize,
  pub grid: Vec<Cell>,
  pub paused: bool
}

impl Game {
  pub fn new(width: usize, height: usize, scale: usize) -> Self {
    let w = width / scale;
    let h = height / scale;
    let size = w * h;
    Game {
      grid: vec![Cell::new(); size],
      width: w,
      height: h,
      scale,
      size,
      paused: true
    }
  }

  pub fn init(&mut self) {
    for c in 0..self.size {
        self.grid[c].init(c, self.width, self.height);
    }
  }

  pub fn toggle_pause(&mut self) {
    self.paused = !self.paused;
  }

  pub fn randomise(&mut self) {
    let mut rng = rand::thread_rng();
    for c in 0..self.size {
      let n: usize = rng.gen();
      self.grid[c].alive = n % 2 == 0;
      self.grid[c].age = 0;
    }
  }

  pub fn count_neighbours(&self, n: usize) -> u8 {
    let mut count = 0;
    let cell = self.grid[n];
    for i in cell.neighbours.iter() {
      let neighbour = self.grid[*i as usize];
      if neighbour.alive {
        count += 1;
      }
    }
    count
  }

  pub fn update(&mut self) {
    // make new grid
    let mut next = vec![Cell::new(); self.size];
    // update cell state
    for (i, cell) in self.grid.iter().enumerate() {
      let alive = cell.alive;
      let neighbours = self.count_neighbours(i);
      // change state based on living neighbours
      if !alive && neighbours == 3 {
          next[i].alive = true;
      } else if (neighbours < 2) || (neighbours > 3) {
          next[i].alive = false;
          next[i].age = 0;
      } else {
          next[i] = self.grid[i];
          next[i].age = cell.age + 1;
      }
      next[i].coords = cell.coords;
      next[i].neighbours = cell.neighbours;
    }
    // recycle grid
    self.grid = next;
  }

  pub fn get_cell(&self, coords: [f64; 2]) -> usize {
    let scale = self.scale as f64;
    let x = (coords[0] / scale ).floor();
    let y = (coords[1] / scale).floor();
    let cell = (y * self.width as f64) + x;
    cell as usize
  }

  pub fn paint(&mut self, coords: [f64; 2]) {
    let index = self.get_cell(coords);
    self.grid[index].toggle_life();
  }
}