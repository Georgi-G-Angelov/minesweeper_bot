use crate::lib::constants::{BOARD_HEIGHT, BOARD_WIDTH, MINE_PERCENTAGE};
use rand::Rng;

pub struct MineBoard {
    pub matrix : [[i32; BOARD_HEIGHT]; BOARD_WIDTH]
}

struct Field {
    x: u8,
    y: u8
}

impl MineBoard {

    pub(crate) fn place_mines(&mut self, x: u8, y: u8) {
        print!("penis: {}, {}", x, y);
        let number_of_mines = BOARD_HEIGHT * BOARD_WIDTH * MINE_PERCENTAGE as usize / 100;
        let mut fields: Vec<Field> = Vec::new();

        // Add possible mines to list
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                if num::abs(i - x as usize) > 1 && num::abs(j - y as usize) > 1{
                    fields.push(Field{ x: i as u8, y: j as u8 });
                }
            }
        }

        // Initialize mines
        let mut rng = rand::thread_rng();
        for i in 0..number_of_mines {
            let field_index = rng.gen(0..fields.len());
            print!("{}", field_index)
        }


    }
}

impl Default for MineBoard {
    fn default() -> Self {
        MineBoard {
            matrix: [[0;BOARD_HEIGHT];BOARD_WIDTH]
        }
    }
}