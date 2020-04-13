use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt;
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LifeGame {
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

    /// update the min and max values of "self"
    fn update_extrema(&mut self) {
        let xs: HashSet<_> = self.board.iter().map(|(x, _)| *x).collect();
        let ys: HashSet<_> = self.board.iter().map(|(_, y)| *y).collect();
        self.max_x = xs.iter().max().cloned().unwrap_or(0);
        self.min_x = xs.iter().min().cloned().unwrap_or(0);
        self.max_y = ys.iter().max().cloned().unwrap_or(0);
        self.min_y = ys.iter().min().cloned().unwrap_or(0);
    }

    /// step takes one step in the given LifeGame.
    pub fn step(&mut self) {
        let mut new_game = self.clone();
        for x in (self.min_x - 1)..(self.max_x + 2) {
            for y in (self.min_y - 1)..(self.max_y + 2) {
                let live_neighbors = self.count_live_neighbors(x, y);
                if self.board.contains(&(x, y)) {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        new_game.board.remove(&(x, y));
                    }
                } else {
                    if live_neighbors == 3 {
                        new_game.board.insert((x, y));
                    }
                }
            }
        }
        new_game.update_extrema();
        *self = new_game.clone();
    }
}

impl fmt::Display for LifeGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: This is a bit hacky. maybe let the board move?
        for x in 0..50 {
            for y in 0..80 {
                let c = if self.board.contains(&(x, y)) {
                    "#"
                } else {
                    " "
                };

                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[test]
fn empty_is_noop() {
    let mut game = LifeGame::new(Vec::new());
    let game_clone = game.clone();
    game.step();
    assert_eq!(game, game_clone);
}

#[test]
fn block() {
    let vec = vec![vec![true, true], vec![true, true]];
    let mut game = LifeGame::new(vec);
    let game_clone = game.clone();
    game.step();
    assert_eq!(game, game_clone);
}

#[test]
fn blinker() {
    let vec = vec![vec![true, true, true]];
    let mut game = LifeGame::new(vec);
    let game_clone = game.clone();
    game.step();
    assert_eq!(
        game,
        LifeGame {
            board: [(-1, 1), (0, 1), (1, 1)].iter().cloned().collect(),
            min_x: -1,
            min_y: 1,
            max_x: 1,
            max_y: 1,
        }
    );
    game.step();
    assert_eq!(
        game,
        LifeGame {
            board: [(0, 0), (0, 1), (0, 2)].iter().cloned().collect(),
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 2,
        }
    );
    assert_eq!(game, game_clone);
}

#[test]
fn glider() {
    let vec = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];

    let mut game = LifeGame::new(vec);
    assert_eq!(
        game,
        LifeGame {
            board: [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)]
                .iter()
                .cloned()
                .collect(),
            min_x: 0,
            min_y: 0,
            max_x: 2,
            max_y: 2,
        }
    );

    game.step();
    assert_eq!(
        game,
        LifeGame {
            board: [(1, 0), (1, 2), (2, 1), (2, 2), (3, 1)]
                .iter()
                .cloned()
                .collect(),
            min_x: 1,
            min_y: 0,
            max_x: 3,
            max_y: 2,
        }
    );

    game.step();
    assert_eq!(
        game,
        LifeGame {
            board: [(1, 2), (2, 0), (2, 2), (3, 1), (3, 2)]
                .iter()
                .cloned()
                .collect(),
            min_x: 1,
            min_y: 0,
            max_x: 3,
            max_y: 2,
        }
    );

    game.step();
    assert_eq!(
        game,
        LifeGame {
            board: [(1, 1), (2, 2), (2, 3), (3, 1), (3, 2)]
                .iter()
                .cloned()
                .collect(),
            min_x: 1,
            min_y: 1,
            max_x: 3,
            max_y: 3,
        }
    );

    game.step();
    assert_eq!(
        game,
        LifeGame {
            board: [(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]
                .iter()
                .cloned()
                .collect(),
            min_x: 1,
            min_y: 1,
            max_x: 3,
            max_y: 3,
        }
    );
}
