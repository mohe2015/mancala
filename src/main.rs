// https://en.wikipedia.org/wiki/Bao_(game)

use std::fmt::Display;

struct Mancala {
    board: [[u8; 16]; 2],
    /*
     0  1  2  3  4  5  6  7
    15 14 13 12 11 10  9  8
    31 30 29 28 27 26 25 24
    16 17 18 19 20 21 22 23

     0  1  2  3  4  5  6  7
    15 14 13 12 11 10  9  8
    15 14 13 12 11 10  9  8
     0  1  2  3  4  5  6  7
    */
}

impl Mancala {
    fn new() -> Self {
        Self {
            // Bao la kujifunza
            board: [[2; 16]; 2],
        }
    }

    // Mtaji phase
    fn mtaji_phase(&mut self, player: usize, position: usize, direction: usize) {
        assert!(self.board[player][position] >= 2);
        // sowing
        let mut i = position;
        let mut count = self.board[player][position];
        self.board[player][position] = 0;
        while count > 0 {
            i += direction % 16;
            self.board[player][i] += 1;
            count -= 1;
        }
        // if marker pit
        if 8 <= position && position <= 15 && self.board[1 - player][position] > 0 {
            // capture -> mtaji turn
            count += self.board[1 - player][position];
            self.board[1 - player][position] = 0;
            // if capture in kimbi and "right" kichwa
            if position == 8 || position == 9 {

            }
            // if capture in kimbi and "left" kichwa
            if position == 14 || position == 15 {

            }
        } else {
            // mtaji turn

        }
    }
}

impl Display for Mancala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            for j in 0..8 {
                write!(f, "{} ", self.board[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");

    let mut mancala = Mancala::new();

    println!("{}", mancala);
}
