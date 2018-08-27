use cell::{Cell, cell_pos};
use column::Column;
use square::Square;
use row::Row;
use ROW_SIZE;
use MIN_CELLS_FILLED;
use print_sudoku;
use get_square_pos;

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

	pub fn inclusive(&mut self) {
		for index  in 0..((ROW_SIZE * ROW_SIZE) as usize) {
			let cell_len = {
				let mut cell = self.cells.get_mut(index).unwrap();
				cell.pos.len()
			};
			if cell_len == 1 {
				let nb = *self.cells.get_mut(index).unwrap().pos.get(0).unwrap();
				self.replace(Some(nb), index as u8);
			}
		}
	}

	pub fn exclusiv(&mut self) {
		for index in 0..((ROW_SIZE * ROW_SIZE) as usize) {
			let mut pos: Vec<u8> = Vec::new();
			let (cell_pos, square_index) = {
				let cell = self.cells.get_mut(index).unwrap();
				(&cell.pos, cell.square)
			};
		}
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
