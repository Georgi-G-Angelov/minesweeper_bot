mod lib {
    pub(crate) mod mineboard;
    mod constants;
}

use crate::lib::mineboard::MineBoard;
use std::io;


fn main() {
    let mut board: MineBoard = MineBoard{..Default::default()};

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let x = input_text.trim().parse::<i32>().unwrap();

    println!("{}", x);


    board.place_mines(0, 0);
    board.print();
}
