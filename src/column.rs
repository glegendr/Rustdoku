use cell::Cell;
use ROW_SIZE;

pub struct Column<'a> {
	pub cells: Vec<&'a Cell>,
	pub index: u8,
}

impl<'a> Column<'a> {
	pub fn new (cells: &'a [Cell], index: u8) -> Self {
		let mut i = 1;
		let mut col: Vec<&'a Cell> = vec![&cells[index as usize]];
		while i < ROW_SIZE {
			col.push(&cells[(index + ROW_SIZE * i) as usize]);
			i = i + 1;
		}
		Self {cells: col, index: index}
	}
	pub fn get_pos(&self) -> Vec<u8> {
		let v: Vec<u8> = self.cells.iter()
			.filter(|x| x.nb.is_some())
			.map(|x| x.nb.unwrap())
			.collect();
		vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter()
			.filter(|x| !v.contains(x))
			.collect()
	}
}
