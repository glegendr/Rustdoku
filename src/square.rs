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
#[cfg(test)]
mod tests {
	use sudoku::*;
	use sudoku::tests::make_grill_test;
#[test]
	fn test_square_new() {
		let sudo = Sudoku::new(make_grill_test());
		let sudo2 = sudo.unwrap();
		let col = Sudoku::square(&sudo2, 0);
		let tmp = col.cells.get(0).unwrap();
		assert_eq!(Some(1), tmp.nb);
		let tmp = col.cells.get(1).unwrap();
		assert_eq!(Some(2), tmp.nb);
		let tmp = col.cells.get(2).unwrap();
		assert_eq!(Some(3), tmp.nb);
		let tmp = col.cells.get(3).unwrap();
		assert_eq!(Some(8), tmp.nb);
		let tmp = col.cells.get(4).unwrap();
		assert_eq!(Some(7), tmp.nb);
		let tmp = col.cells.get(5).unwrap();
		assert_eq!(Some(6), tmp.nb);
		let tmp = col.cells.get(6).unwrap();
		assert_eq!(None, tmp.nb);
		let tmp = col.cells.get(7).unwrap();
		assert_eq!(Some(9), tmp.nb);
		let tmp = col.cells.get(8).unwrap();
		assert_eq!(Some(4), tmp.nb);
	}
}
