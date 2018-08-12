const ROW_SIZE: u8 = 9;

fn get_square(col: u8, row: u8) -> u8 {
	if row < 3 {
		col / 3
	} else if row < 6 {
		3 + col / 3
	} else {
		6 + col / 3
	}
}

#[derive(Debug)]
struct Cell {
	nb: Option<u8>,
	pos: Vec<u8>,
	col: u8,
	row: u8,
	square: u8,
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

struct Sudoku {
	cells: Box<[Cell]>,
}

impl Sudoku {
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


fn main() {
	
}
