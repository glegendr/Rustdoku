extern crate colored;

use colored::*;
use sudoku::Sudoku;
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
		Some(1), Some(2), Some(3), None, None, None, None, None, None,
		None, Some(6), None, None, None, None, None, None, Some(1),
		None, None, None, Some(8), None, None, None, None, None,
		None, None, None, None, Some(7), None, None, None, None,
		None, None, None, None, None, Some(8), None, None, None,
		None, None, None, Some(3), Some(4), None, None, None, None,
		None, Some(3), None, None, None, None, Some(1), Some(7), Some(9),
		None, None, Some(5), None, None, None, Some(6), None, None,
		None, None, None, None, Some(2), None, None, None, None
		];

fn get_square(col: u8, row: u8) -> u8 {
	if row < 3 {
		col / 3
	} else if row < 6 {
		3 + col / 3
	} else {
		6 + col / 3
	}
}

fn print_sudoku(sudoku: Sudoku) {
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
/*
	fn test_col_get_pos() {
		let sudo = Sudoku::new(GRILL);
		let sudo2 = sudo.unwrap();
		let col = Sudoku::column(&sudo2, 2);
		let test = col.get_pos();
		println!("{:?}", test);
	}

	fn test_square_get_pos() {
		let sudo = Sudoku::new(GRILL);
		let sudo2 = sudo.unwrap();
		let sq = Sudoku::square(&sudo2, 0);
		let test = sq.get_pos();
		println!("{:?}", test);
	}

	fn test_row_get_pos() {
		let sudo = Sudoku::new(GRILL);
		let sudo2 = sudo.unwrap();
		let sq = Sudoku::row(&sudo2, 0);
		let test = sq.get_pos();
		println!("{:?}", test);
	}

	fn test_cell_get_pos() {
		let sudo = Sudoku::new(GRILL);
		let mut sudo2 = sudo.unwrap();
		let cell = sudo2.cells.get_mut(0).unwrap();
		cell.get_pos(Sudoku::new(GRILL).unwrap());
		println!("{:?}", cell);
	}
*/
fn main() {
		let sudo = Sudoku::new(GRILL);
		print_sudoku(sudo.unwrap());
//		test_col_get_pos();
//		test_square_get_pos();
//		test_row_get_pos();
//		test_cell_get_pos();
}
