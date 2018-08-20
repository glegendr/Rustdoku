use cell::Cell;
use ROW_SIZE;

pub struct Square<'a> {
	pub cells: Box<[&'a Cell]>,
	pub index: u8,
}

impl<'a> Square<'a> {
	pub fn new(cells: &'a[Cell], index: u8) -> Self {
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
