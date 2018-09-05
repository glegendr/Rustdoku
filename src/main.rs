extern crate colored;

use colored::*;
use sudoku::{Sudoku, SudokuErr};
use row::Row;
use column::Column;
use square::Square;
use cell::Cell;
mod row;
mod square;
mod column;
mod cell;
mod sudoku;
mod test;

const ROW_SIZE: u8 = 9;
const MIN_CELLS_FILLED: usize = 17;
const GRILL: &[Option<u8>] = &[
		None, None, Some(5), Some(4), None, None, None, None, Some(1),
		None, Some(8), None, None, None, Some(6), Some(7), Some(5), None,
		None, None, None, None, Some(3), None, None, Some(9), None,
		None, Some(6), None, None, None, Some(8), Some(4), None, None,
		Some(5), None, None, None, Some(2), Some(1), None, Some(7), None,
		None, None, Some(1), Some(9), None, None, None, None, Some(5),
		None, Some(2), None, None, None, None, Some(3), None, None,
		None, None, Some(4), None, None, None, None, None, Some(2),
		Some(7), None, None, None, None, None, None, Some(4), None
		];

pub fn resolv(mut grill: Sudoku) -> Result<Sudoku, SudokuErr> {
	let mut my_bool = true;
	grill.get_pos();
	while my_bool == true {
		my_bool = false;
		if grill.inclusive() == true {
			grill.get_pos();
			my_bool = true;
		}
		if grill.exclusiv() == true {
			my_bool = true;
		}
	}
	Ok(grill)
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

fn print_sudoku(sudoku: &Sudoku) {
	let mut i = 0;
	for cell in sudoku.cells.iter() {
			let mut print: String = ".".to_string();
		if cell.nb != None {
			print = format!("{}", cell.nb.unwrap());
		}
			let sq = cell.square;
			if sq % 2 == 0 {
				print!("{}", format!("{} ", print.yellow().strikethrough().bold()));
			} else {
				print!("{}", format!("{} ", print.red().bold()));
			}
		i = i + 1;
		if i % 9 == 0 {
			println!("");
		}
	}
}

fn main() {
	let sudo = Sudoku::new(GRILL);
	match sudo {
		Ok(sudoku) => {
			//print_sudoku(&sudoku);
			match resolv(sudoku) {
				Ok(sudoku) => {
					println!();
					print_sudoku(&sudoku);
				},
				Err(er) => println!("Error: Grill not well formated ({:?})", er),
			}
		},
		Err(er) => println!("Error: Grill not well formated ({:?})", er),
	};
}
