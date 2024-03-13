use colored::{Colorize, ColoredString};
use mints::*;
use rand::Rng;

const DEFAULT_PARAMS_EASY: (i32, i32, i32) = (9, 9, 10);
const DEFAULT_PARAMS_MEDIUM: (i32, i32, i32) = (16, 16, 40);
const DEFAULT_PARAMS_HARD: (i32, i32, i32) = (16, 30, 99);

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<Tile>>,
    pub mine_count: i32,
    pub width: i32,
    pub height: i32
}

#[derive(PartialEq, Debug, Clone)]
pub enum Tile {
    Invisible(Box<Tile>),
    Visible(Box<Tile>),
    Flag(Box<Tile>),
    Empty(i32),
    Mine
}

impl Board {
    pub fn new(diff: &Difficulty) -> Board {
        let (width, height, mine_count) = match diff {
            Difficulty::Easy => DEFAULT_PARAMS_EASY,
            Difficulty::Medium => DEFAULT_PARAMS_MEDIUM,
            Difficulty::Hard => DEFAULT_PARAMS_HARD,
        };

        Board {
            board: Self::init_board(width, height, mine_count),
            width,
            height,
            mine_count,
        }
    }

    pub fn init_board(width: i32, height: i32, mine_count: i32) -> Vec<Vec<Tile>> {
        fn assign_mines(board: &mut Vec<Vec<Tile>>, mine_count: i32, width: i32, height: i32) {
            let mut assigned = 0;

            while assigned < mine_count {
                let random_x = rand::thread_rng().gen_range(0..width) as usize;
                let random_y = rand::thread_rng().gen_range(0..height) as usize;

                match &mut board[random_x][random_y] {
                    Tile::Invisible(tile) => {
                        if tile.as_ref().is_empty() {
                            board[random_x][random_y] = Tile::Invisible(Box::new(Tile::Mine));
                            assigned += 1;
                        }
                    }
                    _ => continue
                }
            }
        }

        fn assign_surrounding(board: &mut Vec<Vec<Tile>>, width: i32, height: i32) {
            fn get_surrounding(board: &Vec<Vec<Tile>>, x: usize, y: usize) -> i32 {
                let mut mine_count = 0;

                if board.get(x + 1).is_some_and(|row| row.get(y).is_some_and(|tile| tile.is_mine())) {
                    mine_count += 1;
                }

                if board.get(x - 1).is_some_and(|row| row.get(y).is_some_and(|tile| tile.is_mine())) {
                    mine_count += 1;
                }

                if board.get(x + 1).is_some_and(|row| row.get(y + 1).is_some_and(|tile| tile.is_mine())) {
                    mine_count += 1;
                }

                if board.get(x - 1).is_some_and(|row| row.get(y - 1).is_some_and(|tile| tile.is_mine())) {
                    mine_count += 1;
                }

                if board.get(x + 1).is_some_and(|row| row.get(y - 1).is_some_and(|tile| tile.is_mine())) {
                    mine_count += 1;
                }

                if board.get(x - 1).is_some_and(|row| row.get(y + 1).is_some_and(|tile| tile.is_mine())) {
                    mine_count += 1;
                }

                if board.get(x).is_some_and(|row| row.get(y + 1).is_some_and(|tile| tile.is_mine())) {
                    mine_count += 1;
                }

                if board.get(x).is_some_and(|row| row.get(y - 1).is_some_and(|tile| tile.is_mine())) {
                    mine_count += 1;
                }

                mine_count
            }

            for row in 0..width as usize {
                for col in 0..height as usize {
                    if let Some(tile) = board.get_mut(row).unwrap().get_mut(col) {
                        if tile.is_empty() {
                            board[row][col] = Tile::Invisible(Box::new(Tile::Empty(get_surrounding(board, row, col))));
                        }
                    }
                }
            }
        }

        let mut rows = Vec::new();
        for _ in 0..width {
            let mut cols = Vec::new();
            for _ in 0..height {
                cols.push(Tile::starter_tile());
            }
            rows.push(cols);
        }

        assign_mines(&mut rows, mine_count, width, height);
        assign_surrounding(&mut rows, width, height);

        rows
    }

    pub fn reveal_adjacent(&mut self, x: usize, y: usize) {
        if x >= self.width as usize || y >= self.height as usize {
            return;
        }

        let tile = &self.board[x][y];
        if tile.is_visible() || tile.is_mine() {
            return;
        }

        if let Some(surrounding) = tile.get_empty() {
            self.board[x][y] = Tile::Visible(tile.get_inner());

            if surrounding == 0 {
                self.reveal_adjacent(x + 1, y);
                self.reveal_adjacent(x - 1, y);
                self.reveal_adjacent(x, y + 1);
                self.reveal_adjacent(x, y - 1);
            }
        }
    }

    pub fn check_mine(&self, x: usize, y: usize) -> bool {
        self.board[x][y].is_mine()
    }

    pub fn display(&self, cheat: bool) {
        fn split(num: usize) -> String {
            if num > 9 {
                // let num = num.to_string();
                // format!("{} {}", num.chars().nth(0).unwrap(), num.chars().nth(1).unwrap())
                format!(" {}", num.to_string())
            } else if num > 99 {
                num.to_string()
            } else {
                format!("  {num}")
            }
        }

        let divider = "|---".repeat(self.height as usize + 1);

        println!("{divider}|");
        print!("|   |");
        for row in 0..self.height as usize {
            print!("{}|", split(row + 1).bold());
        }
        newln!();

        println!("{divider}|");
        for (row, rows) in self.board.iter().enumerate() {
            for (col, tile) in rows.iter().enumerate() {
                if col == 0 {
                    print!("|{}|", split(row + 1).bold());
                }
                print!(" {} |", tile.colored_string(cheat));
            }
            newln!();
            println!("{divider}|");
        }
    }
}

impl Tile {
    pub fn starter_tile() -> Tile {
        Tile::Invisible(Box::new(Tile::Empty(0)))
    }

    pub fn is_mine(&self) -> bool {
        match self {
            Tile::Invisible(inner) => inner.is_mine(),
            Tile::Visible(_) => false,
            Tile::Flag(inner) => inner.is_mine(),
            Tile::Empty(_) => false,
            Tile::Mine => true,
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            Tile::Invisible(_) => false,
            Tile::Visible(_) => true,
            Tile::Flag(_) => unreachable!(),
            Tile::Empty(_) => unreachable!(),
            Tile::Mine => unreachable!()
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Tile::Invisible(inner) => inner.is_empty(),
            Tile::Visible(inner) => inner.is_empty(),
            Tile::Flag(inner) => inner.is_empty(),
            Tile::Empty(_) => true,
            Tile::Mine => false,
        }
    }

    pub fn get_empty(&self) -> Option<i32> {
        match self {
            Tile::Invisible(inner) => inner.get_empty(),
            Tile::Visible(inner) => inner.get_empty(),
            Tile::Flag(inner) => inner.get_empty(),
            Tile::Empty(surrounding) => Some(*surrounding),
            Tile::Mine => None,
        }
    }

    pub fn get_inner(&self) -> Box<Tile> {
        match self.clone() {
            Tile::Invisible(inner) => inner,
            Tile::Visible(inner) => inner,
            Tile::Flag(inner) => inner,
            Tile::Empty(_) => unreachable!(),
            Tile::Mine => unreachable!(),
        }
    }

    pub fn colored_string(&self, cheat: bool) -> ColoredString {
        match self {
            Tile::Invisible(inner) => {
                match cheat {
                    true => inner.colored_string(cheat),
                    false => "#".white().bold(),
                }
            },
            Tile::Visible(inner) => inner.colored_string(cheat),
            Tile::Flag(_) => "F".bright_purple().bold(),
            Tile::Empty(surrounding) => {
                match surrounding {
                    0 => " ".white(),
                    1 => "1".blue(),
                    2 => "2".green(),
                    3 => "3".red(),
                    4 => "4".cyan(),
                    5 => "5".magenta(),
                    6 => "6".purple(),
                    7 => "7".yellow(),
                    8 => "8".white(),
                    _ => unreachable!()
                }
            },
            Tile::Mine => "*".bright_red().bold(),
        }
    }
}