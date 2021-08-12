use crate::lib::constants::{BOARD_HEIGHT, BOARD_WIDTH};

pub struct MineBoard {
    pub matrix : [[i32; BOARD_HEIGHT]; BOARD_WIDTH]
}

impl MineBoard {

    pub(crate) fn place_mines(&mut self, x: i32, y: i32) {
        print!("penis: {}, {}", x, y)
    }
}

impl Default for MineBoard {
    fn default() -> Self {
        MineBoard {
            matrix: [[0;BOARD_HEIGHT];BOARD_WIDTH]
        }
    }
}