use std::fmt;


pub struct Sudoku {
    board: [i32; 9],
}

impl Sudoku {
    pub fn new(board: [i32; 9]) -> Sudoku {
        Sudoku { board: board }
    }

    pub fn sum(&self) -> [i32; 8] {
        let mut sum_line: [i32; 8] = [0; 8];

        sum_line[0] = self.board[0] + self.board[1] + self.board[2];
        sum_line[1] = self.board[3] + self.board[4] + self.board[5];
        sum_line[2] = self.board[6] + self.board[7] + self.board[8];

        sum_line[4] = self.board[0] + self.board[3] + self.board[6];
        sum_line[5] = self.board[1] + self.board[4] + self.board[7];
        sum_line[6] = self.board[2] + self.board[5] + self.board[8];
        
        sum_line[7] = self.board[0] + self.board[4] + self.board[8];
        sum_line[3] = self.board[2] + self.board[4] + self.board[6];

        sum_line
    }

    pub fn complete(&self) -> bool {
        let sum_line = self.sum();

        sum_line.iter().all(|&x| x == sum_line[0])
    }

    pub fn swap(&self, i: usize, j: usize) -> Sudoku {
        let mut board_clone = self.board.clone();
        let (e1, e2) = (board_clone[i], board_clone[j]);

        board_clone[j] = e1;
        board_clone[i] = e2;

        Sudoku { board: board_clone }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+---+---+---+\n\
                   | {} | {} | {} | \n\
                   +---+---+---+\n\
                   | {} | {} | {} |\n\
                   +---+---+---+\n\
                   | {} | {} | {} |\n\
                   +---+---+---+", 
               self.board[0], self.board[1], self.board[2], 
               self.board[3], self.board[4], self.board[5], 
               self.board[6], self.board[7], self.board[8])
    }
}
 
