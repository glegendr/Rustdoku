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
	Some(1), None, None, None, None, Some(7), None, Some(9), None,
	None, Some(3), None, None, Some(2), None, None, None, Some(8),
	None, None, Some(9), Some(6), None, None, Some(5), None, None,
	None, None, Some(5), Some(3), None, None, Some(9), None, None,
	None, Some(1), None, None, Some(8), None, None, None, Some(2),
	Some(6), None, None, None, None, Some(4), None, None, None,
	Some(3), None, None, None, None, None, None, Some(1), None,
	None, Some(4), None, None, None, None, None, None, Some(7),
	None, None, Some(7), None, None, None, Some(3), None, None
];

fn recurse(mut sudoku: Sudoku) -> Result<Sudoku, SudokuErr> {
	let mut grill = resolv(sudoku);
	if !grill_full(&grill) {
		let mut pos_index = 0;
		let cell_index = find_first_none(&grill);
		let cell = grill.cells.get(cell_index as usize).unwrap();
		let max = cell.pos.len();
		for pos_index in 0..cell.pos.len() {
			let cell2 = grill.cells.get(cell_index as usize).unwrap();
			let mut pos = *cell2.pos.get(pos_index).unwrap();
			let mut new_grill = grill.clone();
			new_grill.replace(Some(pos), cell_index);
			new_grill.get_pos();
			match recurse(new_grill) {
				Ok(recurs_grill) => return Ok(recurs_grill),
				Err(_) => (),
			}
		}
		Err(SudokuErr::GrillSize)
	} else {
		grill.get_pos();
		Ok(grill)
	}
}

fn find_first_none(sudoku: &Sudoku) -> u8 {
	for i in 0..(ROW_SIZE * ROW_SIZE) {
		let cell = sudoku.cells.get(i as usize).unwrap();
		if cell.nb == None {
			return i;
		}
	}
	0
}

fn try_a_pos(mut sudoku: Sudoku, mut pos: u8) -> Sudoku {
	for i in 0..(ROW_SIZE * ROW_SIZE) {
		let (cell_nb, cell_pos) = {
			let cell = sudoku.cells.get(i as usize).unwrap();
			(cell.nb, cell.pos.clone())
		};
		if cell_nb != None || cell_pos.len() == 0 {
			continue;
		}
		for n_pos in 0..cell_pos.len() {
			if pos == 0 {
				//				println!("{} {} {:?}", pos, n_pos, cell_pos);
				let mut new_sudoku = sudoku.clone();
				new_sudoku.replace(Some(*cell_pos.get(n_pos).unwrap()), i);
				print_sudoku(&new_sudoku);
				println!("");
				return new_sudoku;
			}
			pos -= 1;
		}
	}
	sudoku
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

fn get_pos_max(sudoku: &Sudoku) -> u8 {
	let cells = &sudoku.cells;
	let mut cmpt = 0;
	for i in 0..(ROW_SIZE * ROW_SIZE) {
		let cell = cells.get(i as usize).unwrap();
		if cell.nb == None {
			cmpt += cell.pos.len();
		}
	}
	cmpt as u8
}

fn resolv(mut grill: Sudoku) -> Sudoku {
	let mut my_bool = true;
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
		Ok(mut sudoku) => {
			print_sudoku(&sudoku);
			println!();
			sudoku.get_pos();
			match recurse(sudoku) {
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
