use crate::tetlib::EMP;

#[derive(Clone)]
pub struct Tetrominoe {
    pub shape: Vec<Vec<char>>,
    pub row: usize,
    pub col: usize,
    ptype: char,
    state: usize,
}

impl Tetrominoe {
    pub fn new() -> Tetrominoe {
        Tetrominoe {
            shape: Vec::new(),
            row: 0,
            col: 0,
            ptype: ' ',
            state: 0,
        }
    }

    pub fn set(&mut self, shape: char) {
        self.ptype = shape;
        let shape = match shape {
            'I' => vec![vec![EMP, 'a', EMP, EMP],
                        vec![EMP, 'a', EMP, EMP],
                        vec![EMP, 'a', EMP, EMP],
                        vec![EMP, 'a', EMP, EMP]],

            'J' => vec![vec![EMP, 'a', EMP, EMP],
                        vec![EMP, 'a', EMP, EMP],
                        vec!['a', 'a', EMP, EMP],
                        vec![EMP, EMP, EMP, EMP]],

            'L' => vec![vec![EMP, 'a', EMP, EMP],
                        vec![EMP, 'a', EMP, EMP],
                        vec![EMP, 'a', 'a', EMP],
                        vec![EMP, EMP, EMP, EMP]],

            'O' => vec![vec![EMP, EMP, EMP, EMP],
                        vec![EMP, 'a', 'a', EMP],
                        vec![EMP, 'a', 'a', EMP],
                        vec![EMP, EMP, EMP, EMP]],

            'Z' => vec![vec![EMP, EMP, EMP, EMP],
                        vec!['a', 'a', EMP, EMP],
                        vec![EMP, 'a', 'a', EMP],
                        vec![EMP, EMP, EMP, EMP]],

            'T' => vec![vec![EMP, EMP, EMP, EMP],
                        vec![EMP, 'a', EMP, EMP],
                        vec!['a', 'a', 'a', EMP],
                        vec![EMP, EMP, EMP, EMP]],
                        
            'S' => vec![vec![EMP, EMP, EMP, EMP],
                        vec![EMP, 'a', 'a', EMP],
                        vec!['a', 'a', EMP, EMP],
                        vec![EMP, EMP, EMP, EMP]],
                        
            _ => panic!("Unknown shape: {}", shape),
        };
        self.shape = shape;
        self.state = 0;
    }

    pub fn set_pos(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn rotate(&mut self) {
        match self.ptype {
            'O' => (),
            'I' | 'J' | 'L' | 'T' => {
                // transpose or swap rows and columns
                let n = self.shape.len();
                for i in 0..n {
                    for j in i..n {
                        let temp = self.shape[i][j];
                        self.shape[i][j] = self.shape[j][i];
                        self.shape[j][i] = temp;
                    }
                }
                
                // reverse each row to rotate
                for i in 0..n {
                    self.shape[i].reverse();
                }
            }

        'Z' => {
            if self.state == 0 {
                self.shape = vec![vec![EMP, EMP, EMP, EMP],
                                  vec![EMP, EMP, 'a', EMP],
                                  vec![EMP, 'a', 'a', EMP],
                                  vec![EMP, 'a', EMP, EMP]];
                self.state = 1;
            } else {
                self.shape = vec![vec![EMP, EMP, EMP, EMP],
                                  vec!['a', 'a', EMP, EMP],
                                  vec![EMP, 'a', 'a', EMP],
                                  vec![EMP, EMP, EMP, EMP]];
                self.state = 0;
            }
        }

        'S' => {
            if self.state == 0 {
                self.shape = vec![vec![EMP, EMP, EMP, EMP],
                                  vec![EMP, 'a', EMP, EMP],
                                  vec![EMP, 'a', 'a', EMP],
                                  vec![EMP, EMP, 'a', EMP]];
                
                self.state = 1;
            } else {
                self.shape = vec![vec![EMP, EMP, EMP, EMP],
                                  vec![EMP, 'a', 'a', EMP],
                                  vec!['a', 'a', EMP, EMP],
                                  vec![EMP, EMP, EMP, EMP]];
                self.state = 0;
            }
        }

        _ => panic!("Unknown shape: {}", self.ptype),
    }
}
    
}