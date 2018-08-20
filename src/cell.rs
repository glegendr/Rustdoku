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
	pub fn get_pos(&mut self, sud: Sudoku) {
		let col = sud.column(self.col);
		let row = sud.row(self.row);
		let square = sud.square(self.square);
		self.pos = Vec::new();
		self.pos.extend(col.get_pos());
		self.pos.extend(row.get_pos());
		self.pos.extend(square.get_pos());
		self.pos.sort();
		let mut i = 1;
		let mut v = Vec::new();
		while i < 10 {
			if self.pos.iter().filter(|&x| *x == i).count() == 3 {
				v.push(i);
			}
			i = i + 1;
		}
		self.pos = v;
	}
}
