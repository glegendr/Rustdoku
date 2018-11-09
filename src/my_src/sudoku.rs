use my_src::cell::{Cell, cell_pos};
use my_src::column::Column;
use my_src::square::Square;
use my_src::row::Row;
use ROW_SIZE;
use MIN_CELLS_FILLED;

#[derive(Debug)]
pub enum SudokuErr {
	GrillSize,
	MultResult,
	ImpossibleGrill
}

#[derive(Clone, Debug)]
pub struct Sudoku {
	pub cells: Box<[Cell]>,
}

    impl PartialEq for Sudoku {
    fn eq(&self, other: &Sudoku) -> bool {
        self.cells == other.cells
    }
}

impl Sudoku {
		/// create a new sudoku with the vector grill.
		/// # Example
		///
		/// ```
		/// let sudoku: Sudoku = Sudoku::new(grill);
		/// ```
	pub fn new(grill: Vec<Option<u8>>) -> Result<Self, SudokuErr> {
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


		/// replace a cell with the number 'nb' at index 'index'.
		/// # Example
		///
		/// ```
		/// let sudoku = Sudoku::new(grill);
		/// sudoku.replace(Some(8), 0);
		/// sudoku.replace(None, 1);
		///
		/// assert_eq!(Some(8), sudoku.cells.get(0).unwrap().nb);
		/// assert_eq!(None, sudoku.cells.get(1).unwrap().nb);
		/// ```
	pub fn replace(&mut self, nb: Option<u8>, index: u8) {
		if let Some(elem) = self.cells.get_mut(index as usize) {
			elem.nb = nb;
		}
	}

		/// Get the posibility for all cells in the Sudoku.
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
		/// let sudoku = Sudoku::new(grill);
		/// sudoku.get_pos();
		/// assert_eq!(vec![2, 5, 6, 8], sudoku.cells.get(1).unwrap().pos);
		/// ```
	pub fn get_pos(&mut self) {
		for index  in 0..((ROW_SIZE * ROW_SIZE) as usize) {
			cell_pos(index, self);
		}
	}


		/// resolv with inclusiv method.
		///
		/// The inclusiv method places the numbers if there is only this possibility.
		///
		/// Return true if a number as been placed
		/// # Example
		///
		/// ```
		/// grill:
		/// ...1.3..9
		/// ...456..7
		/// ...789..3
		/// ........2
		/// ........1
		/// 1235476.8
		/// ........4
		/// .........
		/// ........6
		///
		/// ...
		///
		/// let sudoku = Sudoku::new(grill);
		/// sudoku.get_pos();
		/// assert_eq!(true, sudoku.inclusive());
		/// assert_eq!(Some(2), sudoku.cells.get(4).unwrap().nb);
		/// ```
	pub fn inclusive(&mut self) -> bool {
		let mut my_bool = false;
		for index in 0..((ROW_SIZE * ROW_SIZE) as usize) {
			let (cell_len, cell_nb) = {
				let mut cell = self.cells.get_mut(index).unwrap();
				(cell.pos.len(), cell.nb)
			};
			if cell_nb != None {
				continue;
			}
			if cell_len == 1 {
				let nb = *self.cells.get_mut(index).unwrap().pos.get(0).unwrap();
				self.replace(Some(nb), index as u8);
				my_bool = true;
			}
		}
		return my_bool;
	}


		/// resolv with exclusiv method.
		///
		/// The inclusiv method places the numbers if there is only this place in a square/column/row.
		///
		/// Return true if a number as been placed
		/// # Example
		///
		/// ```
		/// grill:
		/// 1..8.....
		/// .5...2...
		/// 78.......
		/// .92......
		/// .....1...
		/// ........8
		/// .........
		/// ..1....8.
		/// .........
		///
		/// ...
		///
		/// let sudoku = Sudoku::new(grill);
		/// sudoku.get_pos();
		/// assert_eq!(true, sudoku.exclusiv());
		/// assert_eq!(Some(2), sudoku.cells.get(1).unwrap().nb);
		/// ```
	pub fn exclusiv(&mut self) -> bool {
		let mut my_bool = false;
		for index in 0..((ROW_SIZE * ROW_SIZE) as usize) {
			if self.exclusiv_sq(index) == true {
				self.get_pos();
				my_bool = true;
			}
			if self.exclusiv_row(index) == true {
				self.get_pos();
				my_bool = true;
			}
			if self.exclusiv_col(index) == true {
				self.get_pos();
				my_bool = true;
			}
		}
		return my_bool;
	}

	fn exclusiv_col(&mut self, index: usize) -> bool {
		let mut vec: Vec<u8> = Vec::new();
		let (col, nb, mut pos) = {
			let cell = self.cells.get_mut(index).unwrap();
			(cell.col, cell.nb, cell.pos.clone())
		};
		if nb != None {
			return false;
		}
		for index2 in 0..((ROW_SIZE * ROW_SIZE) as usize) {
			if index2 == index {
				continue;
			}
			let (col2, nb2, pos2) = {
				let cell = self.cells.get_mut(index2).unwrap();
				(cell.col, cell.nb, cell.pos.clone())
			};
			if col2 == col && nb2 == None{
				vec.extend(pos2);
			}
		}
		vec.sort();
		vec.dedup();
		for i in 0..vec.len() {
			let x = vec.get(i);
			for ind in 0..pos.len() {
				if x == pos.get(ind) {
					pos.remove(ind);
				}
			}
		}
		if pos.len() == 1 {
			let change: u8 = *pos.get(0).unwrap();
			self.replace(Some(change), index as u8);
			return true;
		}
		return false;
	}

	fn exclusiv_row(&mut self, index: usize) -> bool {
		let mut vec: Vec<u8> = Vec::new();
		let (row, nb, mut pos) = {
			let cell = self.cells.get_mut(index).unwrap();
			(cell.row, cell.nb, cell.pos.clone())
		};
		if nb != None {
			return false;
		}
		for index2 in 0..((ROW_SIZE * ROW_SIZE) as usize) {
			if index2 == index {
				continue;
			}
			let (row2, nb2, pos2) = {
				let cell = self.cells.get_mut(index2).unwrap();
				(cell.row, cell.nb, cell.pos.clone())
			};
			if row2 == row && nb2 == None{
				vec.extend(pos2);
			}
		}
		vec.sort();
		vec.dedup();
		for i in 0..vec.len() {
			let x = vec.get(i);
			for ind in 0..pos.len() {
				if x == pos.get(ind) {
					pos.remove(ind);
				}
			}
		}
		if pos.len() == 1 {
			let change: u8 = *pos.get(0).unwrap();
			self.replace(Some(change), index as u8);
			return true;
		}
		return false;
	}

	fn exclusiv_sq(&mut self, index: usize) -> bool {
		let mut vec: Vec<u8> = Vec::new();
		let (sq, nb, mut pos) = {
			let cell = self.cells.get_mut(index).unwrap();
			(cell.square, cell.nb, cell.pos.clone())
		};
		if nb != None {
			return false;
		}
		for index2 in 0..((ROW_SIZE * ROW_SIZE) as usize) {
			if index2 == index {
				continue;
			}
			let (sq2, nb2, pos2) = {
				let cell = self.cells.get_mut(index2).unwrap();
				(cell.square, cell.nb, cell.pos.clone())
			};
			if sq2 == sq && nb2 == None{
				vec.extend(pos2);
			}
		}
		vec.sort();
		vec.dedup();
		for i in 0..vec.len() {
			let x = vec.get(i);
			for ind in 0..pos.len() {
				if x == pos.get(ind) {
					pos.remove(ind);
				}
			}
		}
		if pos.len() == 1 {
			let change: u8 = *pos.get(0).unwrap();
			self.replace(Some(change), index as u8);
			return true;
		}
		return false;
	}

		/// this function will return the column at index 'index'
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
		/// let sudoku: Sudoku = Sudoku::new(grill);
		/// let col: Column = sudoku.column(1);
		/// assert_eq!(1, col.index);
		/// // col.cells == None, Some(3), None, None, Some(1), None, None, Some(4), None
		/// ```
	pub fn column(&self, index: u8) -> Column {
		Column::new(&self.cells, index)
	}

		/// this function will return the row at index 'index'
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
		/// let sudoku: Sudoku = Sudoku::new(grill);
		/// let row: Row = sudoku.row(1);
		/// let vec = vec![None, Some(3), None, None, Some(2), None, None, None, Some(8)];
		/// let box = vec.into_boxed_slice();
		/// assert_eq!(box, row.row);
		/// ```
	pub fn row(&self, index: u8) -> Row {
		Row::new(&self.cells, index)
	}

		/// this function will return the square  at index 'index'
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
		/// let sudoku: Sudoku = Sudoku::new(grill);
		/// let sq: Square = sudoku.square(0);
		/// let vec = vec![Some(1), None, None, None, Some(3), None, None, None, Some(9)];
		/// let box = vec.into_boxed_slice();
		/// assert_eq!(box, square.cells);
		/// assert_eq!(0, square.index);
		/// ```
	pub fn square(&self, index: u8) -> Square {
		Square::new(&self.cells, index)
	}
}

#[cfg(test)]
pub mod tests {
	use super::*;
	pub fn make_grill_test() -> Vec<Option<u8>> {
	vec![
Some(1), Some(2), Some(3), None, None, None, None, None, None,
Some(8), Some(7), Some(6), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6),
None, Some(9), Some(4), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3),
Some(9), Some(6), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8),
Some(6), None, Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5),
Some(3), None, Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2),
None, None, None, Some(2), Some(3), Some(4), Some(5), None, Some(7),
Some(5), None, Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4),
Some(2), None, Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1)
]
	}
#[test]
	fn test_sudoku_new() {
		let sudo = Sudoku::new(make_grill_test());
		assert!(sudo.is_ok());
		let sudo2 = sudo.unwrap();
		assert_eq!(3, sudo2.cells[2].nb.unwrap());
		assert_eq!(2, sudo2.cells[2].col);
		assert_eq!(0, sudo2.cells[2].row);
		assert_eq!(0, sudo2.cells[2].square);
	}
}
