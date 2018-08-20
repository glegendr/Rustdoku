use cell::Cell;
use column::Column;
use square::Square;
use row::Row;
use ROW_SIZE;
use MIN_CELLS_FILLED;

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
