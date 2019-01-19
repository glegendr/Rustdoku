extern crate colored;
use self::colored::*;
extern crate rayon;
use rayon::prelude::*;
use std::fmt;
mod my_src;
pub use my_src::sudoku::{Sudoku, SudokuErr};
const ROW_SIZE: u8 = 9;
const MIN_CELLS_FILLED: usize = 17;

#[derive(Clone, Debug)]
pub struct Rustdoku {
    sudoku: Sudoku,
}

impl Rustdoku {
    /// This is creating the rustdoku and give it
    /// # Example
    ///
    /// ```
    /// let mut doku: Rustdoku = Rustdoku::new(grill).unwrap();
    /// ```
    pub fn new(grill: Vec<Option<u8>>) -> Result<Self, SudokuErr> {
        match Sudoku::new(grill) {
            Ok(mut sudo) => {
                sudo.get_pos();
                Ok(Self {sudoku: sudo})
            },
            Err(er) => Err(er),
        }
    }

    /// This is the entry of this library, here it launch the algorithm.
    /// # Example
    ///
    /// ```
    /// grill:
    /// 1....7.9.
    /// .3..2...8
    /// ..96..5..
    /// ..53..9..
    /// .1..8...2
    /// 6....4...
    /// 3......1.
    /// .4......7
    /// ..7...3..
    ///
    /// ...
    ///
    /// let mut doku: Rustdoku = Rustdoku::new(grill).unwrap();
    /// match doku.solve() {
    ///		Ok(rustdoku) => println!("{}", rustdoku),
    ///		Err(er) => println!("{:?}", er),
    /// };
    /// // It will display:
    /// // 1 6 2 8 5 7 4 9 3
    /// // 5 3 4 1 2 9 6 7 8
    /// // 7 8 9 6 4 3 5 2 1
    /// // 4 7 5 3 1 2 9 8 6
    /// // 9 1 3 5 8 6 7 4 2
    /// // 6 2 8 7 9 4 1 3 5
    /// // 3 5 6 4 7 8 2 1 9
    /// // 2 4 1 9 3 5 8 6 7
    /// // 8 9 7 2 6 1 3 5 4
    /// ```
    pub fn solve(&mut self, flags: (bool, bool)) -> Result<Self, SudokuErr> {
        self.sudoku.get_pos();
        let cpy = self.sudoku.clone();
        let par: Vec<Sudoku> = vec![cpy, self.sudoku.clone()];
        let mut first_result: Vec<Result<Sudoku, SudokuErr>> = par.par_iter().enumerate().map(|(i, x)| recurse(x, i)).collect();
        let result: Result<Sudoku, SudokuErr> =  first_result.pop().unwrap();
        let result2: Result<Sudoku, SudokuErr> =  first_result.pop().unwrap();
        match result {
            Ok(sudo) => {
                match result2 {
                    Ok(sudo2) => {
                        if sudo2 != sudo && flags.1 == true {
				if flags.0 == true {
					println!("{}\n\n{}", sudo, sudo2);
				}
                        	return Err(SudokuErr::MultResult);
                        }
                        self.sudoku = sudo.clone();
                        Ok(self.clone())
                    },
                    Err(_) => {
                        self.sudoku = sudo.clone();
                        Ok(self.clone())
                    }
                }
            },
            Err(er) => Err(er),
        }
    }
}

impl fmt::Display for Rustdoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        let mut str: String = String::new();
        for cell in self.sudoku.cells.iter() {
            let mut print: String = ".".to_string();
            if cell.nb != None {
                print = format!("{}", cell.nb.unwrap());
            }
            let sq = cell.square;
            if sq % 2 == 0 {
                str.push_str(format!("{} ", print.yellow().strikethrough().bold()).as_str());
            } else {
                str.push_str(format!("{} ", print.red().bold()).as_str());
            }
            i = i + 1;
            if i % 9 == 0  && i != 81 {
                str.push('\n');
            }
        }
        write!(f, "{}", str)
    }
}


pub fn recurse(base: &Sudoku, my_bool: usize) -> Result<Sudoku, SudokuErr> {
    let mut grill = resolv(base.clone());
    if !grill_full(&grill) {
        let cell_index = find_first_none(&grill, my_bool);
        let cell = grill.cells.get(cell_index as usize).unwrap().clone();
            for pos_index in 0..cell.pos.len() {
                let cell2 = grill.cells.get(cell_index as usize).unwrap();
                let mut pos = *cell2.pos.get(pos_index).unwrap();
                let mut new_grill = grill.clone();
                new_grill.replace(Some(pos), cell_index);
                new_grill.get_pos();
                match recurse(&new_grill, 0) {
                    Ok(recurs_grill) => return Ok(recurs_grill),
                    Err(_) => (),
                }
            }
        Err(SudokuErr::ImpossibleGrill)
    } else {
        grill.get_pos();
        Ok(grill)
    }
}

fn find_first_none(sudoku: &Sudoku, my_bool: usize) -> u8 {
    let mut ret: Vec<(u8, u8)> = Vec::new();
    for i in 0..(ROW_SIZE * ROW_SIZE) {
        let cell = sudoku.cells.get(i as usize).unwrap();
        if cell.nb == None {
            let tmp: (u8, u8) = (i, (cell.pos.len() as u8));
            ret.push(tmp);
        }
    }
    ret.sort_by(|a, b| a.1.cmp(&b.1));
    if ret.len() != 0 {
    let arr: (u8, u8) = *ret.get(0).unwrap();
    if my_bool == 0 {
    	arr.0
    } else {
	    arr.0 + arr.1 - 1 
	    }
    } else {
	    0
    }
}

fn grill_full(sudoku: &Sudoku) -> bool {
    for i in 0..(ROW_SIZE * ROW_SIZE) {
        let (col, row, sq) = {
            let cell = sudoku.cells.get(i as usize).unwrap();
            (sudoku.column(cell.col), sudoku.row(cell.row), sudoku.square(cell.square))
        };
        let mut tmp = col.get_pos();
        if tmp.len() != 0 {
            return false;
        }
        tmp = row.get_pos();
        if tmp.len() != 0 {
            return false;
        }
        tmp = sq.get_pos();
        if tmp.len() != 0 {
            return false;
        }
    }
    true
}

fn resolv(mut grill: Sudoku) -> Sudoku {
    let mut my_bool = true;
    while my_bool == true {
        my_bool = false;
	grill.get_pos();
        if grill.inclusive() == true {
            my_bool = true;
	} else if grill.exclusiv() == true {
            my_bool = true;
        }
    }
    grill
}

fn get_square(col: u8, row: u8) -> u8 {
    if row < 3 {
        col / 3
    } else if row < 6 {
        3 + col / 3
    } else {
        6 + col / 3
    }
}

#[cfg(test)]
mod tests {
    use grill_full;
    use recurse;
    use my_src::sudoku::*;
#[test]
    fn test_grill_full() {
        let grill_true: Vec<Option<u8>> = vec![
            Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9),
            Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6),
            Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3),
            Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8),
            Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5),
            Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2),
            Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7),
            Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4),
            Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1)
        ];
        let grill_false = vec![
            Some(1), Some(3), Some(2), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9),
            Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6),
            Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3),
            Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8),
            Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5),
            Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2),
            Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7),
            Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4),
            Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1)
        ];
        let sudo_true = Sudoku::new(grill_true);
        let sudo_false = Sudoku::new(grill_false);
        let sudo2_true = sudo_true.unwrap();
        let sudo2_false = sudo_false.unwrap();
        assert_eq!(true, grill_full(&sudo2_true));
        assert_eq!(false, grill_full(&sudo2_false));
    }

#[feature(test)]
extern crate test;
use self::Bencher;

     #[bench]
    fn bench_sudoku(b: &mut Bencher) {
        let grill_true: Vec<Option<u8>> = vec![
            Some(1), None, None, None, None, Some(7), None, Some(9), None,
            None, Some(3), None, None, Some(2), None, None, None, Some(8),
            None, None, Some(9), Some(6), None, None, Some(5), None, None,
            None, None, Some(5), Some(3), None, None, Some(6), None, None,
            None, Some(1), None, None, Some(8), None, None, None, Some(2),
            Some(6), None, None, None, None, Some(4), None, None, None,
            Some(3), None, None, None, None, None, None, Some(1), None,
            None, Some(4), Some(1), None, None, None, None, None, Some(7),
            None, None, Some(7), None, None, None, Some(3), None, None
        ];
        let sudo_true = Sudoku::new(grill_true);
        let sudo2_true = sudo_true.unwrap();
        b.iter(|| recurse(&sudo2_true, 1));
    }
}
