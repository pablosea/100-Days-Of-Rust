use rand::{self, Rng};

/// Represents the four possible walls of a cell
#[derive(Clone, Copy, Debug)]
enum Wall {
    Top,
    Right,
    Bottom,
    Left,
}

impl Wall {
    fn opposite(&self) -> Wall {
        match self {
            Wall::Top => Wall::Bottom,
            Wall::Right => Wall::Left,
            Wall::Bottom => Wall::Top,
            Wall::Left => Wall::Right,
        }
    }
}

/// Represents a single cell in the maze
#[derive(Clone, Copy)]
pub struct Cell {
    visited: bool,
    walls: [bool; 4],
}

impl Cell {
    /// Creates a new cell with all walls intact and not visited
    pub fn new() -> Cell {
        Cell {
            visited: false,
            walls: [true; 4],
        }
    }

    fn has_wall(&self, wall: Wall) -> bool {
        self.walls[wall as usize]
    }

    fn remove_wall(&mut self, wall: Wall) {
        self.walls[wall as usize] = false;
    }
}

/// Represents the entire maze
pub struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Maze {
    /// Creates a new maze with the specified dimensions
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the maze
    /// * `height` - The height of the maze
    ///
    /// # Errors
    ///
    /// Returns an error if width or height is 0
    pub fn new(width: usize, height: usize) -> Result<Maze, &'static str> {
        if width == 0 || height == 0 {
            return Err("Maze dimensions must be greater than 0");
        }
        let cells = (0..width * height).map(|_| Cell::new()).collect();
        Ok(Maze {
            width,
            height,
            cells,
        })
    }

    /// Gets a reference to the cell at the specified coordinates
    pub fn get(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y * self.width + x]
    }

    /// Gets a mutable reference to the cell at the specified coordinates
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y * self.width + x]
    }

    /// Gets the valid neighboring coordinates of a cell
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

fn generate_maze(maze: &mut Maze) {
    let mut rng = rand::thread_rng();
    let mut stack = Vec::new();
    let (mut x, mut y) = (0, 0);
    maze.get_mut(x, y).visited = true;

    loop {
        let neighbors = maze.get_neighbors(x, y);
        let unvisited_neighbors: Vec<_> = neighbors
            .iter()
            .filter(|&&(nx, ny)| !maze.get(nx, ny).visited)
            .collect();

        if !unvisited_neighbors.is_empty() {
            stack.push((x, y));
            let &(next_x, next_y) = unvisited_neighbors[rng.gen_range(0..unvisited_neighbors.len())];
            
            let wall_to_remove = if next_x < x {
                Wall::Left
            } else if next_x > x {
                Wall::Right
            } else if next_y < y {
                Wall::Top
            } else {
                Wall::Bottom
            };

            maze.get_mut(x, y).remove_wall(wall_to_remove);
            maze.get_mut(next_x, next_y).remove_wall(wall_to_remove.opposite());

            x = next_x;
            y = next_y;
            maze.get_mut(x, y).visited = true;
        } else if let Some((prev_x, prev_y)) = stack.pop() {
            x = prev_x;
            y = prev_y;
        } else {
            break;
        }
    }
}

/// Prints the maze to the console
fn print_maze(maze: &Maze) {
    for y in 0..maze.height {
        // Print the top walls
        for x in 0..maze.width {
            print!("+");
            if maze.get(x, y).has_wall(Wall::Top) || y == 0 {
                print!("---");
            } else {
                print!("   ");
            }
        }
        println!("+");

        // Print the side walls and spaces
        for x in 0..maze.width {
            if maze.get(x, y).has_wall(Wall::Left) {
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

fn main() -> Result<(), &'static str> {
    let height = 5;
    let width = 7;
    let mut maze = Maze::new(width, height)?;
    generate_maze(&mut maze);
    print_maze(&maze);
    Ok(())
}