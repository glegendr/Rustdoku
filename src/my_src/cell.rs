use my_src::sudoku::Sudoku;
use ROW_SIZE;
use get_square;

#[derive(Debug, Eq, Clone)]
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
	if nb != None {
		let cell = sud.cells.get_mut(index).unwrap();
		cell.pos.clear();
		return;
	}
	let (col, row, square) = {
		let col = sud.column(col_index).get_pos();
		let row = sud.row(row_index).get_pos();
		let square = sud.square(square_index).get_pos();
		(col, row, square)
	};
	{
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
	}

	for size in 2..4 {
		let mut v = Vec::new();
		for index2 in 0..((ROW_SIZE * ROW_SIZE) as usize) {
			let (square_index2, row_index2, col_index2, nb2) = {
				let cell = sud.cells.get_mut(index2).unwrap();
				(cell.square, cell.row, cell.col, cell.nb)
			};
			if nb2 != None {
				continue;
			}
			if square_index == square_index2 {
				v.extend(del_square(sud, square_index, size));
			}
			if row_index == row_index2 {
				v.extend(del_row(sud, row_index, size));
			}
			if col_index == col_index2 {
				v.extend(del_col(sud, col_index, size));
			}
		}
		if v.len() == 0 {
			continue;
		}
		v.sort();
		v.dedup();
		let cell = sud.cells.get_mut(index).unwrap();
		let mut tmp = cell.pos.clone();
		for i in 0..v.len() {
			let x = v.get(i);
			for ind in 0..tmp.len() {
				if x == tmp.get(ind) {
					tmp.remove(ind);
				}
			}
		}
		if tmp.len() == 0 {
			continue;
		}
		cell.pos = tmp;
	}
}

fn del_square(sud: &mut Sudoku, square_index: u8, size: usize) -> Vec<u8> {
	let mut del: Vec<u8> = Vec::new();
	let mut tmp: Vec<Vec<u8>> = Vec::new();
	let square = sud.square(square_index);
	for i in 0..ROW_SIZE {
		let cell = square.cells.get(i as usize).unwrap();
		let pos: Vec<u8> = cell.pos.clone();
		if pos.len() == size {
			tmp.push(pos);
		}
	}
	for y in 0..tmp.len() {
		let count = tmp.iter().filter(|&x| *x == *tmp.get(y).unwrap()).count();
		if count == size {
			for i in 0..size {
				del.push(*tmp.get(y).unwrap().get(i).unwrap());
			}
		}
	}
	del.sort();
	del.dedup();
	//	if square_index == 2 && size == 3 {
	//		println!("{}: {:?}", square_index, del);
	//	}
	return del;
}

fn del_row(sud: &mut Sudoku, row_index: u8, size: usize) -> Vec<u8> {
	let mut del: Vec<u8> = Vec::new();
	let mut tmp: Vec<Vec<u8>> = Vec::new();
	let row = sud.row(row_index);
	for i in 0..ROW_SIZE {
		let cell = row.row.get(i as usize).unwrap();
		let pos: Vec<u8> = cell.pos.clone();
		if pos.len() == size {
			tmp.push(pos);
		}
	}
	for y in 0..tmp.len() {
		let count = tmp.iter().filter(|&x| *x == *tmp.get(y).unwrap()).count();
		if count == size {
			for i in 0..size {
				del.push(*tmp.get(y).unwrap().get(i).unwrap());
			}
		}
	}
	del.sort();
	del.dedup();
	//	if del.len() != 0 {
	//		println!("{}: {:?}", row_index, del);
	//	}
	return del;
}

fn del_col(sud: &mut Sudoku, col_index: u8, size: usize) -> Vec<u8> {
	let mut del: Vec<u8> = Vec::new();
	let mut tmp: Vec<Vec<u8>> = Vec::new();
	let col = sud.column(col_index);
	for i in 0..ROW_SIZE {
		let cell = col.cells.get(i as usize).unwrap();
		let pos: Vec<u8> = cell.pos.clone();
		if pos.len() == size {
			tmp.push(pos);
		}
	}
	for y in 0..tmp.len() {
		let count = tmp.iter().filter(|&x| x == tmp.get(y).unwrap()).count();
		if col_index == 6  && size == 3 {
			//		println!("col:{} -> {}", col_index, count);
		}
		if count == size {
			for i in 0..size {
				del.push(*tmp.get(y).unwrap().get(i).unwrap());
			}
		}
	}
	del.sort();
	del.dedup();
	//	if col_index == 6 && size == 3 {
	//		println!("{}: {:?}", col_index, del);
	//	}
	return del;
}

#[cfg(test)]
mod tests {
	use super::*;

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
}
