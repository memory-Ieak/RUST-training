pub mod board;

pub fn run() {
    let board = board::Board();
    println!("{}", board);
}