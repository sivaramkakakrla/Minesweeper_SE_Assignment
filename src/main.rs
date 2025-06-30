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
}

impl Board {
    fn new() -> Self {
        Board {
            grid: vec![vec![Cell::new(); WIDTH]; HEIGHT],
            game_over: false,
            mines_placed: false,
        }
    }

    fn place_mines(&mut self){
        let mut rng = rand::thread_rng();
        let mut mines_to_place = NUM_MINES;
        while mines_to_place > 0 {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            
            if self.grid[x][y].has_mine == true{
                continue;
            }
            self.grid[x][y].has_mine = true;
            mines_to_place -= 1;
        }
        self.mines_placed = true;
    }

    fn uncover(&mut self, x: usize, y: usize) -> bool {
        if self.grid[x][y].has_mine == true {
            self.game_over = true;
            return false;
        }

        self.grid[x][y].state = uncovered;
        let mut count = 0;
        // hab=ndling (0 0) case separatly
        if x==0 && y ==0 {
            if self.grid[0][1].has_mine{
                count+=1; }
            if self.grid[1][0].has_mine{
                    count+=1; }
            if self.grid[1][1].has_mine{
                count+=1; }

                for row in &self.grid {
                    for cell in row {
                        if cell.has_mine {
                            print!("*");
                        } else if cell.state == uncovered{
                            print!("{}",count);
                        }
                        else {
                            print!("?");
                        }
                        print!(" ");
                    }
                    println!();
                }
                return true;
        }
        if x-1>=0 {
            if self.grid[x-1][y].has_mine{
            count+=1; }
        }
        if x+1<=4 {
            if self.grid[x+1][y].has_mine {
            count+=1; }
        }
        if y+1<=4 {
            if self.grid[x][y+1].has_mine {
            count+=1;}
        }
        if y-1>=0 {
            if self.grid[x][y-1].has_mine {
            count+=1; }
        }
        if x-1>=0 && y-1>=0 {
            if self.grid[x-1][y-1].has_mine {
            count+=1;}
        }
        if x+1<=4 && y-1>=0 {
            if self.grid[x+1][y-1].has_mine {
            count+=1;}
        }
        if x+1<=4 && y+1<=4 {
            if self.grid[x+1][y+1].has_mine {
            count+=1; }
        }
        if x-1>=0 && y+1<=4 {
            if self.grid[x-1][y+1].has_mine {
            count+=1; }
        }       

        for row in &self.grid {
            for cell in row {
                if cell.has_mine {
                    print!("*");
                } else if cell.state == uncovered{
                    print!("{}",count);
                }
                else {
                    print!("?");
                }
                print!(" ");
            }
            println!();
        }
        true
    }

    fn check_win(&self) -> bool {
        let uncovered_cells = self.grid.iter().flatten().filter(|c| c.state == uncovered).count();
        uncovered_cells == WIDTH * HEIGHT - NUM_MINES
    }

    fn print_mines(&self) {
        for row in &self.grid {
            for cell in row {
                if cell.has_mine == true {
                    print!("*");
                }else {
                    print!("?");
                }
                print!(" ");
            }
            println!();
        }
        
    }
}

fn main() {
    let mut board = Board::new();
    //placing the mines and print 
    board.place_mines(); // m - mine
    board.print_mines(); // m - mine
    loop {        
        if board.game_over {
            println!("You hit a mine Game over.");
            break;
        }

        if board.check_win() {
            println!("Congratulations You have cleared all the mines");
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
       
        board.uncover(x, y);
    }
}
