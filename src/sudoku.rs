use cell::{Cell, cell_pos};
use column::Column;
use square::Square;
use row::Row;
use ROW_SIZE;
use MIN_CELLS_FILLED;
use print_sudoku;

#[derive(Debug)]
pub enum SudokuErr {
	GrillSize,
	MultResult,
}

pub struct Sudoku {
	pub cells: Box<[Cell]>,
}

impl Sudoku {
	pub fn new(grill: &[Option<u8>]) -> Result<Self, SudokuErr> {
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

	pub fn replace(&mut self, nb: Option<u8>, index: u8) {
		if let Some(elem) = self.cells.get_mut(index as usize) {
			elem.nb = nb;
		}
	}

	pub fn get_pos(&mut self) {
		for index  in 0..((ROW_SIZE * ROW_SIZE) as usize) {
			cell_pos(index, self);
		}
	}

	pub fn inclusive(&mut self) -> bool {
		let mut my_bool = false;
		for index  in 0..((ROW_SIZE * ROW_SIZE) as usize) {
			let cell_len = {
				let mut cell = self.cells.get_mut(index).unwrap();
				cell.pos.len()
			};
			if cell_len == 1 {
				let nb = *self.cells.get_mut(index).unwrap().pos.get(0).unwrap();
				self.replace(Some(nb), index as u8);
				my_bool = true;
			}
		}
		return my_bool;
	}

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

	pub fn column(&self, index: u8) -> Column {
		Column::new(&self.cells, index)
	}

	pub fn row(&self, index: u8) -> Row {
		Row::new(&self.cells, index)
	}

	pub fn square(&self, index: u8) -> Square {
		Square::new(&self.cells, index)
	}
}
