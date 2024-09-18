use rand::{self, Rng};

pub struct Cell {
    visited: bool,
    walls: [bool; 4],
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            visited: false,
            walls: [true, true, true, true],
        }
    }
    
}

pub struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut cells = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            cells.push(Cell::new());
        }
        Maze {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y * self.width + x]
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }
    
}

fn draw_maze(height: usize, width: usize) {
    let mut maze = Maze::new(width, height);
    let mut rng = rand::thread_rng();
    let mut stack = Vec::new();
    let mut x = 0;
    let mut y = 0;
    maze.get_mut(x, y).visited = true;
    loop {
        let neighbors = maze.get_neighbors(x, y);
        let mut unvisited_neighbors = Vec::new();
        for &(nx, ny) in &neighbors {
            if !maze.get(nx, ny).visited {
                unvisited_neighbors.push((nx, ny));
            }
        }
        if !unvisited_neighbors.is_empty() {
            stack.push((x, y));
            let (nx, ny) = unvisited_neighbors[rng.gen_range(0..unvisited_neighbors.len())];
            let current_index = neighbors.iter().position(|&(cx, cy)| cx == nx && cy == ny).unwrap();
            let neighbor_index = maze.get_neighbors(nx, ny).iter().position(|&(cx, cy)| cx == x && cy == y).unwrap();
            maze.get_mut(x, y).walls[current_index] = false;
            maze.get_mut(nx, ny).walls[neighbor_index] = false;
            x = nx;
            y = ny;
            maze.get_mut(x, y).visited = true;
        } else if let Some((nx, ny)) = stack.pop() {
            x = nx;
            y = ny;
        } else {
            break;
        }
    }    
    
    // Print the maze with correct formatting
    for y in 0..maze.height {
        // Print the top walls
        for x in 0..maze.width {
            print!("+");
            if maze.get(x, y).walls[0] || y == 0 {
                print!("---");
            } else {
                print!("   ");
            }
        }
        println!("+");

        // Print the side walls and spaces
        for x in 0..maze.width {
            if maze.get(x, y).walls[3] {
                print!("|");
            } else {
                print!(" ");
            }
            print!("   ");
        }
        println!("|");
    }

    // Print the bottom walls
    for _x in 0..maze.width {
        print!("+---");
    }
    println!("+");
}

fn main() {
    draw_maze(5,7);
}