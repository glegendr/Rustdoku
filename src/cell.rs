use sudoku::Sudoku;
use ROW_SIZE;
use get_square;

#[derive(Debug, Eq)]
pub struct Cell {
	pub nb: Option<u8>,
	pub pos: Vec<u8>,
	pub col: u8,
	pub row: u8,
	pub square: u8,
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
	pub fn new(index: u8, nb: Option<u8>) -> Self {
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
}

pub fn cell_pos(index: usize, sud: &mut Sudoku) {
	let (col_index, row_index, square_index, nb) = {
		let mut cell = sud.cells.get_mut(index).unwrap();
		(cell.col, cell.row, cell.square, cell.nb)
		};
	let (col, row, square) = {
		let col = sud.column(col_index).get_pos();
		let row = sud.row(row_index).get_pos();
		let square = sud.square(square_index).get_pos();
		(col, row, square)
	};
	let cell = sud.cells.get_mut(index).unwrap();
	cell.pos = Vec::new();
	cell.pos.extend(col);
	cell.pos.extend(row);
	cell.pos.extend(square);
	cell.pos.sort();
	let mut v = Vec::new();
	for i in 1..=ROW_SIZE {
		if cell.pos.iter().filter(|&x| *x == i).count() == 3 {
			v.push(i);
		}
	}
	cell.pos = v;
	if nb != None {
		cell.pos.clear();
	}
	if square_index == 1 {
		println!("{:?}", cell.pos);
	}
}
