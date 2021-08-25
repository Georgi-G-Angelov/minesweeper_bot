use crate::lib::constants::{BOARD_HEIGHT, BOARD_WIDTH, MINE_PERCENTAGE};
use rand::Rng;
use std::{cmp, io};
use crate::lib::util::read_int;

pub struct MineBoard {
    /*
        hidden:
        0 - unknown
        1 - uncovered
        2 - marked as bomb
     */
    pub matrix : [[i32; BOARD_WIDTH]; BOARD_HEIGHT],
    pub hidden : [[i8; BOARD_WIDTH]; BOARD_HEIGHT]
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

        // Uncover fields
        self.uncover_fields()
    }

    fn uncover_fields(&mut self) {
        for i in 0..BOARD_HEIGHT {
            for j in 0..BOARD_WIDTH {
                if self.matrix[i][j] == 0 {
                    self.hidden[i][j] = 1;
                    for x in -1..2 {
                        for y in -1..2 {
                            let i1: i16 = i as i16;
                            let j1: i16 = j as i16;
                            if i1 + x >= 0 && i1 + x < BOARD_HEIGHT as i16 &&
                                j1 + y >= 0 && j1 + y < BOARD_WIDTH as i16 {
                                self.hidden[(i1 + x) as usize][(j1 + y) as usize] = 1;
                            }
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn print(&mut self) {
        for i in 0..BOARD_HEIGHT {
            for j in 0..BOARD_WIDTH {
                if self.hidden[i as usize][j as usize] == 0 {
                    print!(" x ")
                } else if self.hidden[i as usize][j as usize] == 1 {
                    if self.matrix[i as usize][j as usize] != -1 {
                        print!(" ")
                    }
                    print!("{} ", self.matrix[i as usize][j as usize]);
                } else {
                    print!(" # ")
                }
            }
            println!();
        }
    }

    pub(crate) fn mark(&mut self) -> bool {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read from stdin");

        let y = read_int() as usize;
        let x = read_int() as usize;

        if command.trim().eq("mark") {
            if self.hidden[x][y] == 2 {
                self.hidden[x][y] = 0;
            } else if self.hidden[x][y] == 0{
                self.hidden[x][y] = 2;
            }
        } else if command.trim().eq("uncover") {
            if self.matrix[x][y] == -1 {
                return true;
            } else {
                self.hidden[x][y] = 1;
            }
        }
        return false;
    }
}

impl Default for MineBoard {
    fn default() -> Self {
        MineBoard {
            matrix: [[0;BOARD_WIDTH];BOARD_HEIGHT],
            hidden: [[0; BOARD_WIDTH]; BOARD_HEIGHT]
        }
    }
}