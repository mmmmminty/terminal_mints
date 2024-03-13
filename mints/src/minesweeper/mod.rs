use std::error::Error;
use std::str::SplitAsciiWhitespace;
use std::time::Instant;
use std::io::{Write, BufReader, BufRead};

use mints::*;

mod utils;

use utils::*;

pub struct Minesweeper {
    pub board: Board,
    pub turn: i32,
    pub time_started: Instant,
    pub difficulty: Difficulty
}

impl Game for Minesweeper {
    fn new(mut args: Args) -> Self {
        Minesweeper::parse_args(&mut args);
        Minesweeper {
            board: Board::new(args.difficulty.as_ref().unwrap()),
            turn: 0,
            time_started: Instant::now(),
            difficulty: args.difficulty.unwrap().clone()
        }
    }

    fn start(&mut self) {
        titled_loading_screen("minesweeper", "white", 800);
        newln!();
    }
    
    fn do_loop(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        clear!();
        self.board.display(false);

        let mut input = BufReader::new(std::io::stdin()).lines();
        let cmd = match input.next() {
            Some(s) => s.unwrap(),
            None => return Ok(GAME_OVER),
        };

        let code = self.handle_commands(&cmd);
        self.turn += 1;

        Ok(code)
    }

    fn finish(self) {
        println!("Thanks for playing!");
        drop(self);
    }
}

impl Minesweeper {
    fn parse_args(args: &mut Args) {
        if args.difficulty.is_none() {
            args.difficulty = Some(Difficulty::Easy);
        }
    }

    fn handle_commands(&mut self, cmd: &String) -> i32 {
        fn parse_next(split: &mut SplitAsciiWhitespace, expected: usize) -> Result<Vec<String>, Box<dyn Error>> {
            let mut args = Vec::new();
            for _ in 0..expected {
                let arg = split.next().ok_or("Parsing more arguments than expected!")?;
                args.push(arg.to_string());
            }
            
            Ok(args)
        }

        fn parse_coords(args: &[String]) -> Result<(usize, usize), Box<dyn Error>> {
            let x: usize = args[0].parse()?;
            let y: usize = args[1].parse()?;

            // 1 is subtracted as the display board is not 0-indexed.
            Ok((x - 1, y - 1))
        }

        let mut split = cmd.split_ascii_whitespace();
        match split.nth(0).unwrap().to_ascii_lowercase().as_str() {
            "!check" | "!c" => {
                let args = match parse_next(&mut split, 2) {
                    Ok(args) => args,
                    Err(_e) => {
                        println!("{cmd} requires 2 arguments (E.g. {cmd} 3 4)");
                        return GAME_ONGOING;
                    }
                };

                let (x, y) = match parse_coords(&args) {
                    Ok(coords) => coords,
                    Err(_) => {
                        println!("Make sure {cmd} is formatted correctly! (E.g. {cmd} 3 4)");
                        return GAME_ONGOING;
                    },
                };
                self.check(x, y)
            }
            "!flag" | "!f" => {
                todo!();
            }
            "!unflag" | "!u" => {
                todo!();
            }
            "!cheat" => {
                clear!();
                self.board.display(true);
                sleep!(3000);
                GAME_ONGOING
            }
            // Below are the default commands:
            "!hint" | "!h" => {
                GAME_ONGOING
            }
            "!restart" | "!next" | "!reset" | "!r" => {
                print!("Restarting in 3.. ");
                flush!();
                sleep!(1000);
                print!("2.. ");
                flush!();
                sleep!(1000);
                print!("1.. ");
                flush!();
                sleep!(1000);

                GAME_RESTART
            }
            "!quit" | "!leave" | "!exit" | "!q" => GAME_OVER,
            _ => {
                println!("Unknown command!");
                GAME_ONGOING
            }
        }
    }

    fn check(&mut self, x: usize, y: usize) -> i32 {
        if self.board.check_mine(x, y) {
            clear!();
            println!("That was a mine! Unlucky!");
            newln!(2);
            self.board.display(true);
            GAME_OVER

        } else {
            self.board.reveal_adjacent(x, y);
            GAME_ONGOING
        }
    }
}