mod lib {
    pub(crate) mod mineboard;
    mod constants;
    pub(crate) mod util;
}

use crate::lib::mineboard::MineBoard;
use crate::lib::util::read_int;

fn main() {
    let mut board: MineBoard = MineBoard{..Default::default()};

    let x = read_int();
    let y = read_int();

    board.place_mines(x, y);

    loop {
        board.print();
        if board.mark() {
            break;
        }
    }
}
