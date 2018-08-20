use cell::Cell;
use ROW_SIZE;

pub struct Row<'a> {
	pub row: &'a [Cell],
}

impl<'a> Row<'a> {
	pub fn new(cells: &'a[Cell], index: u8) -> Self {
		Self {row: &cells[(index * ROW_SIZE) as usize .. (index * ROW_SIZE + ROW_SIZE) as usize]}
	}
	pub fn get_pos(&self) -> Vec<u8> {
		let v: Vec<u8> = self.row.iter()
			.filter(|x| x.nb.is_some())
			.map(|x| x.nb.unwrap())
			.collect();
		vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter()
			.filter(|x| !v.contains(x))
			.collect()
	}
}
