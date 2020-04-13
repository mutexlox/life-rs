use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

mod life;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let contents = if args.len() == 1 || args[1] == "-" {
        io::stdin().lock().lines().collect::<Result<Vec<_>, _>>()?
    } else {
        let file = File::open(&args[1])?;
        BufReader::new(file)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
    };

    // Read the input, interpreting whitespace as false and non-whitespace as true
    let mut cells = Vec::new();
    for line in contents.iter() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(!c.is_whitespace())
        }
        cells.push(row);
    }

    let mut game = life::LifeGame::new(cells);
    for _ in 0..1000 {
        println!("{}", game);
        sleep(Duration::from_millis(500));
        game.step();
    }
    Ok(())
}
