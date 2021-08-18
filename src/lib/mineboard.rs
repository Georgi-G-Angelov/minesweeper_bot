use crate::lib::constants::{BOARD_HEIGHT, BOARD_WIDTH, MINE_PERCENTAGE};
use rand::Rng;
use std::{cmp, io};

pub struct MineBoard {
    pub matrix : [[i32; BOARD_WIDTH]; BOARD_HEIGHT]
}

struct Field {
    pub x: u8,
    pub y: u8
}

impl MineBoard {

    pub(crate) fn place_mines(&mut self, x: u8, y: u8) {
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
            let field = fields.get(field_index).unwrap();
            self.matrix[field.y as usize][field.x as usize] = -1;
            fields.remove(field_index);
        }

        // Fill field values around mines
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                if self.matrix[j][i] == 0 {
                    let lower_x_range = cmp::max(0, i as i8 - 1);
                    let upper_x_range = cmp::min(BOARD_WIDTH as i8, i as i8 + 2);
                    let lower_y_range = cmp::max(0, j as i8 - 1);
                    let upper_y_range = cmp::min(BOARD_HEIGHT as i8, j as i8 + 2);
                    let mut mine_number = 0;
                    for x in lower_x_range..upper_x_range {
                        for y in lower_y_range..upper_y_range {
                            if self.matrix[y as usize][x as usize] == -1 {
                                mine_number+=1;
                            }
                        }
                    }
                    self.matrix[j][i] = mine_number;
                }
            }
        }
    }

    pub(crate) fn print(&mut self) {
        for i in 0..BOARD_HEIGHT {
            for j in 0..BOARD_WIDTH {
                if self.matrix[i as usize][j as usize] != -1 {
                    print!(" ")
                }
                print!("{} ", self.matrix[i as usize][j as usize]);
            }
            println!();
        }
    }

    pub(crate) fn mark(&mut self) -> bool {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read from stdin");

        if command.eq("mark") {

        } else if command.eq("uncover") {

        }
        return false;
    }
}

impl Default for MineBoard {
    fn default() -> Self {
        MineBoard {
            matrix: [[0;BOARD_WIDTH];BOARD_HEIGHT]
        }
    }
}