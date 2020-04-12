use std::collections::HashSet;
use std::convert::TryInto;
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
struct LifeGame {
    board: HashSet<(i32, i32)>,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl LifeGame {
    pub fn new(state: Vec<Vec<bool>>) -> Self {
        let mut board = HashSet::new();
        let mut max_x: i32 = 0;
        let mut max_y: i32 = 0;
        for (x, row) in state.iter().enumerate() {
            for (y, live) in row.iter().enumerate() {
                if *live {
                    let x_i32 = x.try_into().unwrap();
                    let y_i32 = y.try_into().unwrap();
                    board.insert((x_i32, y_i32));
                    if x_i32 > max_x {
                        max_x = x_i32;
                    }
                    if y_i32 > max_y {
                        max_y = y_i32;
                    }
                }
            }
        }
        LifeGame {
            board,
            min_x: 0,
            min_y: 0,
            max_x,
            max_y,
        }
    }

    fn count_live_neighbors(&self, x: i32, y: i32) -> usize {
        let mut live = 0;
        for n_x in (x - 1)..(x + 2) {
            for n_y in (y - 1)..(y + 2) {
                if !(n_x == x && n_y == y) {
                    if self.board.contains(&(n_x, n_y)) {
                        live += 1;
                    }
                }
            }
        }

        live
    }

    /// step takes one step in the given game of life
    pub fn step(&mut self) {
        for x in self.min_x..self.max_x {
            for y in self.min_y..self.max_y {
                let live_neighbors = self.count_live_neighbors(x, y);
                if self.board.contains(&(x, y)) {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        self.board.remove(&(x, y));
                    }
                } else {
                    if live_neighbors == 3 {
                        self.board.insert((x, y));
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
