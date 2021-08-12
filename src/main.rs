mod lib {
    pub(crate) mod mineboard;
    mod constants;
}

use crate::lib::mineboard::MineBoard;

fn main() {
    let mut board: MineBoard = MineBoard{..Default::default()};
    board.place_mines(1, 2);
}
