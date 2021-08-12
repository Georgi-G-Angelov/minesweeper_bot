use crate::lib::constants::{BOARD_HEIGHT, BOARD_WIDTH, MINE_PERCENTAGE};
use rand::Rng;

pub struct MineBoard {
    pub matrix : [[i32; BOARD_WIDTH]; BOARD_HEIGHT]
}

struct Field {
    pub x: u8,
    pub y: u8
}

impl MineBoard {

    pub(crate) fn place_mines(&mut self, x: u8, y: u8) {
        print!("penis: {}, {}", x, y);
        let number_of_mines = BOARD_HEIGHT * BOARD_WIDTH * MINE_PERCENTAGE as usize / 100;
        let mut fields: Vec<Field> = Vec::new();

        // Add possible mines to list
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                let x_diff = i as i16 - x as i16;
                let y_diff = j as i16 - y as i16;
                if x_diff.abs() > 1 || y_diff.abs() > 1{
                    fields.push(Field{ x: i as u8, y: j as u8 });
                }
            }
        }

        // Initialize mines
        let mut rng = rand::thread_rng();
        for _i in 0..number_of_mines {
            let field_index = rng.gen_range(0..fields.len());
            println!("{}, {}", unsafe { fields.get_unchecked(field_index) }.x,
                     unsafe { fields.get_unchecked(field_index) }.y);
            let field = fields.get(field_index).unwrap();
            self.matrix[field.x as usize][field.y as usize] = -1;
            fields.remove(field_index);
        }
    }

    pub(crate) fn print(&mut self) {
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                if self.matrix[i as usize][j as usize] != -1 {
                    print!(" ")
                }
                print!("{} ", self.matrix[i as usize][j as usize]);
            }
            println!();
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