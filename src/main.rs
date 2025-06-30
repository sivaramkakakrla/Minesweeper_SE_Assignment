use std::io;
use rand::Rng;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
const NUM_MINES: usize = 5;
const uncovered: u8 = 10;
const covered: u8 = 11;
const unknown: u8 = 9;


#[derive(Debug, Clone, Copy)]
struct Cell {
    state: u8,
    has_mine: bool,
    adjacent_mines: u8,
}

impl Cell {
    fn new() -> Self {
        Cell {
            state: unknown,
            has_mine: false,
            adjacent_mines: 0,
        }
    }
}

struct Board {
    grid: Vec<Vec<Cell>>,
    game_over: bool,
    mines_placed: bool,
    first_move: Option<(usize, usize)>,
}

impl Board {
    fn new() -> Self {
        Board {
            grid: vec![vec![Cell::new(); WIDTH]; HEIGHT],
            game_over: false,
            mines_placed: false,
            first_move: None,
        }
    }

    fn print_board(&self) {
        print!("  ");
        for x in 0..WIDTH {
            print!("{}", x);
        }
        println!();
        for y in (0..HEIGHT).rev() {
            print!("{}|", y);
            for x in 0..WIDTH {
                let cell = &self.grid[x][y];
                if cell.state == covered || cell.state == unknown || cell.has_mine {
                    print!("?");
                } else if cell.state == uncovered {
                    if cell.adjacent_mines > 0 {
                        print!("{}", cell.adjacent_mines);
                    } else {
                        print!(" ");
                    }
                } else {
                    print!("?");
                }
            }
            println!();
        }
        // Print list of mine coordinates
        let mines = self.mine_list();
        print!("Mines:");
        for (x, y) in mines {
            print!(" ({} {})", x, y);
        }
        println!();
    }

    fn mine_list(&self) -> Vec<(usize, usize)> {
        let mut mines = Vec::new();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.grid[x][y].has_mine {
                    mines.push((x, y));
                }
            }
        }
        mines
    }

    fn place_mines(&mut self, first_x: usize, first_y: usize) {
        let mut rng = rand::thread_rng();
        let mut mines_to_place = NUM_MINES;
        while mines_to_place > 0 {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            if (x == first_x && y == first_y) || self.grid[x][y].has_mine {
                continue;
            }
            self.grid[x][y].has_mine = true;
            mines_to_place -= 1;
        }
        self.calculate_adjacent_mines();
        self.mines_placed = true;
    }

    fn calculate_adjacent_mines(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let mut count = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                            if self.grid[nx as usize][ny as usize].has_mine {
                                count += 1;
                            }
                        }
                    }
                }
                self.grid[x][y].adjacent_mines = count;
            }
        }
    }

    fn uncover(&mut self, x: usize, y: usize) -> bool {
        if self.grid[x][y].state == uncovered {
            return true;
        }
        if self.grid[x][y].has_mine {
            self.grid[x][y].state = uncovered;
            self.game_over = true;
            return false;
        }
        self.recursive_uncover(x, y);
        true
    }

    fn recursive_uncover(&mut self, x: usize, y: usize) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }
        if self.grid[x][y].state == uncovered {
            return;
        }
        self.grid[x][y].state = uncovered;
        if self.grid[x][y].adjacent_mines > 0 {
            return;
        }
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                    self.recursive_uncover(nx as usize, ny as usize);
                }
            }
        }
    }

    fn check_win(&self) -> bool {
        let mut covered_cells = 0;
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if !self.grid[x][y].has_mine && self.grid[x][y].state != uncovered {
                    covered_cells += 1;
                }
            }
        }
        covered_cells == 0
    }
}

fn main() {
    let mut board = Board::new();
    board.print_board();
    loop {
        if board.game_over {
            println!("You hit a mine. Game over.");
            // Optionally, reveal all mines
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    if board.grid[x][y].has_mine {
                        board.grid[x][y].state = uncovered;
                    }
                }
            }
            board.print_board();
            break;
        }
        if board.check_win() {
            println!("Congratulations You have cleared all the mines.");
            board.print_board();
            break;
        }
        println!("Enter coordinates (x y):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coords: Vec<Result<usize, _>> = input.trim().split_whitespace().map(|s| s.parse()).collect();
        if coords.len() != 2 {
            println!("Invalid input. Please enter coordinates in the format 'x y'.");
            continue;
        }
        let x = match coords[0] {
            Ok(val) if val < WIDTH => val,
            _ => {
                println!("Invalid x coordinate.");
                continue;
            }
        };
        let y = match coords[1] {
            Ok(val) if val < HEIGHT => val,
            _ => {
                println!("Invalid y coordinate.");
                continue;
            }
        };
        if !board.mines_placed {
            board.place_mines(x, y);
        }
        board.uncover(x, y);
        board.print_board();
    }
}
