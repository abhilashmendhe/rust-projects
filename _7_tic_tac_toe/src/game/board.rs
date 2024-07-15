use std::collections::HashMap;

use super::player::Player;

#[derive(Debug, Clone)]
pub struct Board {
    size: usize,
    board_matrix: Vec<Vec<char>>,
    row_count: HashMap<(usize, char), usize>,
    col_count: HashMap<(usize, char), usize>,
    diag_count: HashMap<char, usize>,
    rev_diag_count: HashMap<char, usize>,
    total: usize
}

impl Board {
    pub fn new(size: usize) -> Board {
        let row = vec!['_'; size as usize];
        let mut mat = vec![];
        for _ in 0..size {
            mat.push(row.clone());
        }

        Board {
            size: size,
            board_matrix: mat,
            row_count: HashMap::new(),
            col_count: HashMap::new(),
            diag_count: HashMap::new(),
            rev_diag_count: HashMap::new(),
            total: 0
        }
    }

    pub fn place(&mut self, player: Player, x: usize, y: usize) -> bool {
        let mut f = false;
        self.total += 1;
        if self.total == (self.size*self.size) {
            panic!("No more places left. End game!!!");
        }
        let mark = player.marker;

        if  x >= self.size || y >= self.size {
            panic!("Marker can't be placed. Position give are out of bounds.");
        }

        let present_already = self.board_matrix[x][y];

        if present_already != '_' {
            panic!("Already marker present {}",present_already);
        }
        
        // check row-count for marker 'x' or 'o' depending on player's marker
        match self.row_count.get(&(x, mark)) {
            Some(val) => {
                if *val == self.size - 1 {
                    f = true;
                }
                self.row_count.insert((x, mark), val + 1);
            },
            None => {
                self.row_count.insert((x, mark), 1);
            }
        }
        
        // check column-count
        match self.col_count.get(&(y, mark)) {
            Some(val) => {
                if *val == self.size - 1 {
                    f = true;
                }
                self.col_count.insert((y, mark), val + 1);
            },
            None => {
                self.col_count.insert((y, mark), 1);
            }
        }

        // diagonal count
        if x == y {
            match self.diag_count.get(&(mark)) {
                Some(val) => {
                    if *val == self.size - 1 {
                        f = true;
                    }
                    self.diag_count.insert(mark, val + 1);
                },
                None => {
                    self.diag_count.insert(mark, 1);
                }
            }
        }

        // reverse diagonal count
        if (x+y) == self.size - 1 {
            match self.rev_diag_count.get(&(mark)) {
                Some(val) => {
                    if *val == self.size - 1 {
                        f = true;
                    }
                    self.rev_diag_count.insert(mark, val + 1);
                },
                None => {
                    self.rev_diag_count.insert(mark, 1);
                }
            }
        }

        self.board_matrix[x][y] = mark;

        f
    }

    pub fn print_board(&self) {
        for i in 0..self.size {
            println!("{:?}",self.board_matrix[i]);
        }
    }
}