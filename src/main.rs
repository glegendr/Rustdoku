const ROW_SIZE: u8 = 9;
const MIN_CELLS_FILLED: usize = 17;

fn get_square(col: u8, row: u8) -> u8 {
	if row < 3 {
		col / 3
	} else if row < 6 {
		3 + col / 3
	} else {
		6 + col / 3
	}
}

#[derive(Debug, Eq)]
struct Cell {
	nb: Option<u8>,
	pos: Vec<u8>,
	col: u8,
	row: u8,
	square: u8,
}

impl PartialEq for Cell {
	fn eq(&self, other: &Cell) -> bool {
		self.nb == other.nb
		&& self.col == other.col
		&& self.row == other.row
		&& self.square == other.square
	}
}

impl Cell {
	fn new(index: u8, nb: Option<u8>) -> Self {
		let col = index % ROW_SIZE;
		let row = index / ROW_SIZE;
		Self {
		nb: nb,
		pos: Vec::new(),
		col: col,
		row: row,
		square: get_square(col, row),
		}
	}
	fn get_pos(&mut self, sud: Sudoku) {
		let col = sud.column(self.col);
		let row = sud.row(self.row);
		let square = sud.square(self.square);
		self.pos = Vec::new();
		self.pos.extend(col.get_pos());
		self.pos.extend(row.get_pos());
		self.pos.extend(square.get_pos());
		self.pos.sort();
		self.pos.dedup();
	}
}

struct Column<'a> {
	cells: &'a [Cell],
	index: u8,
}

impl<'a> Column<'a> {
	fn new (cells: &'a [Cell], index: u8) -> Self {
		Self {cells: cells, index: index}
	}
	fn get_pos(&self) -> Vec<u8> {
		let v: Vec<u8> = self.cells.iter()
			.skip(self.index as usize)
			.step_by(ROW_SIZE as usize)
			.filter(|x| x.nb.is_some())
			.map(|x| x.nb.unwrap())
			.collect();
		vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter()
			.filter(|x| !v.contains(x))
			.collect()
	}
}

struct Row<'a> {
	row: &'a [Cell],
}

impl<'a> Row<'a> {
	fn new(cells: &'a[Cell], index: u8) -> Self {
		Self {row: &cells[(index * ROW_SIZE) as usize .. (index * ROW_SIZE + ROW_SIZE) as usize]}
	}
	fn get_pos(&self) -> Vec<u8> {
		let v: Vec<u8> = self.row.iter()
			.filter(|x| x.nb.is_some())
			.map(|x| x.nb.unwrap())
			.collect();
		vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter()
			.filter(|x| !v.contains(x))
			.collect()
	}
}

struct Square<'a> {
	cells: Box<[&'a Cell]>,
	index: u8,
}

impl<'a> Square<'a> {
	fn new(cells: &'a[Cell], index: u8) -> Self {
		let mut square = Vec::with_capacity(9);
		let row_begin = match index {
			0 ... 2 => 0,
			3 ... 5 => 3,
			6 ... 8 => 6,
			_ => unreachable!(),
		};
		let col_begin = (index % 3) * 3;
		for i in 0..3 {
			let local_begin = (row_begin + i) * ROW_SIZE + col_begin;
			for cell in cells[local_begin as usize..(local_begin + 3) as usize].iter() {
				square.push(cell);
			}
		}
		Self {cells: square.into_boxed_slice(), index: index}
	}
	fn get_pos(&self) -> Vec<u8> {
		let v: Vec<u8> = self.cells.iter()
			.filter(|x| x.nb.is_some())
			.map(|x| x.nb.unwrap())
			.collect();
		vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter()
			.filter(|x| !v.contains(x))
			.collect()
	}
}
#[derive(Debug)]
enum SudokuErr {
	GrillSize,
	MultResult,
}

struct Sudoku {
	cells: Box<[Cell]>,
}

impl Sudoku {
	fn new(grill: &[Option<u8>]) -> Result<Self, SudokuErr> {
		if grill.iter().filter(|x| x.is_some()).count() < MIN_CELLS_FILLED {
			return Err(SudokuErr::MultResult);
		}
		if grill.len() != (ROW_SIZE * ROW_SIZE) as usize {
			return Err(SudokuErr::GrillSize);
		}
		let vec: Vec<Cell>= grill.iter().enumerate()
			.map(|(i, x)| Cell::new(i as u8, *x))
			.collect();
		Ok (Self {cells: vec.into_boxed_slice()})
	}
	fn column(&self, index: u8) -> Column {
		Column::new(&self.cells, index)
	}
	fn row(&self, index: u8) -> Row {
		Row::new(&self.cells, index)
	}
	fn square(&self, index: u8) -> Square {
		Square::new(&self.cells, index)
	}
}

use std::env;
fn main() {
	let mut i = 0;
	let v: Vec<Option<u8>>;
for argument in env::args() {
	if i != 0 {
		println!("{}", argument);
	}
	i = 1;
}
}

#[cfg(test)]
mod tests {
	use super::*;

#[test]
	fn test_square_1() {
		assert_eq!(0, get_square(0, 0));
	}
#[test]
	fn test_square_2() {
		assert_eq!(1, get_square(3, 1));
	}
#[test]
	fn test_square_3() {
		assert_eq!(2, get_square(6, 2));
	}
#[test]
	fn test_square_4() {
		assert_eq!(3, get_square(2, 4));
	}
#[test]
	fn test_square_5() {
		assert_eq!(4, get_square(5, 5));
	}
#[test]
	fn test_square_6() {
		assert_eq!(5, get_square(7, 5));
	}
#[test]
	fn test_square_7() {
		assert_eq!(6, get_square(0, 7));
	}
#[test]
	fn test_square_8() {
		assert_eq!(7, get_square(4, 8));
	}
#[test]
	fn test_square_9() {
		assert_eq!(8, get_square(6, 6));
	}
#[test]
	fn test_cell_new() {
		let col = 3;
		let row = 3;
		let index = 30;
		let square = 4;
		let nb = 6;

		let cell = Cell::new(index, Some(nb));
		assert_eq!(col, cell.col);
		assert_eq!(row, cell.row);
		assert_eq!(nb, cell.nb.unwrap());
		assert_eq!(square, cell.square);
	}
#[test]
	fn test_sudoku_new() {
		let grill: &[Option<u8>] = &[
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
		let sudo = Sudoku::new(grill);
		assert!(sudo.is_ok());
	/*	match sudo {
			Err(SudokuErr::GrillSize) => assert_eq!(1, 2),
			Err(SudokuErr::MultResult) => assert_eq!(3, 4),
			Ok(sud) => {
				assert_eq!(3, sud.cells[2].nb.unwrap());
				assert_eq!(2, sud.cells[2].col);
				assert_eq!(0, sud.cells[2].row);
				assert_eq!(0, sud.cells[2].square);
			},
		}*/
			let sudo2 = sudo.unwrap();
				assert_eq!(3, sudo2.cells[2].nb.unwrap());
				assert_eq!(2, sudo2.cells[2].col);
				assert_eq!(0, sudo2.cells[2].row);
				assert_eq!(0, sudo2.cells[2].square);
	}
}
