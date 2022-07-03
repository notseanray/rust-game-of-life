mod board;
use board::Board;
use std::{thread::sleep, time::Duration};

pub fn run() {
    let mut board = Board::new();
    board.fill_random();
    println!("{board}");
    loop {
        board.tick();
        println!("{board}");
        sleep(Duration::from_millis(400));
    }
}
