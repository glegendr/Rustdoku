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
#[cfg(test)]
mod tests {
	use sudoku::*;
	use sudoku::tests::make_grill_test;
#[test]
	fn test_col_new() {
		let sudo = Sudoku::new(make_grill_test());
		let sudo2 = sudo.unwrap();
		let col = Sudoku::column(&sudo2, 1);
		let tmp = col.cells.get(0).unwrap();
		assert_eq!(Some(2), tmp.nb);
		let tmp = col.cells.get(1).unwrap();
		assert_eq!(Some(7), tmp.nb);
		let tmp = col.cells.get(2).unwrap();
		assert_eq!(Some(9), tmp.nb);
		let tmp = col.cells.get(3).unwrap();
		assert_eq!(Some(6), tmp.nb);
		let tmp = col.cells.get(4).unwrap();
		assert_eq!(None, tmp.nb);
		let tmp = col.cells.get(5).unwrap();
		assert_eq!(None, tmp.nb);
		let tmp = col.cells.get(6).unwrap();
		assert_eq!(None, tmp.nb);
		let tmp = col.cells.get(7).unwrap();
		assert_eq!(None, tmp.nb);
		let tmp = col.cells.get(8).unwrap();
		assert_eq!(None, tmp.nb);
	}
#[test]
	fn test_col_pos() {
		let sudo = Sudoku::new(make_grill_test());
		let sudo2 = sudo.unwrap();
		let col = Sudoku::column(&sudo2, 1);
		let test = col.get_pos();
		let tmp = *test.get(0).unwrap();
		assert_eq!(1, tmp);
		let tmp = *test.get(1).unwrap();
		assert_eq!(3, tmp);
		let tmp = *test.get(2).unwrap();
		assert_eq!(4, tmp);
		let tmp = *test.get(3).unwrap();
		assert_eq!(5, tmp);
		let tmp = *test.get(4).unwrap();
		assert_eq!(8, tmp);
	}
}
