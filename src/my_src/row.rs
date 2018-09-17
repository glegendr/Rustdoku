use my_src::cell::Cell;
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

#[cfg(test)]
mod tests {
	use lib::sudoku::*;
	use lib::sudoku::tests::make_grill_test;
#[test]
	fn test_row_new() {
		let sudo = Sudoku::new(make_grill_test());
		let sudo2 = sudo.unwrap();
		let col = Sudoku::row(&sudo2, 0);
		let tmp = col.row.get(0).unwrap();
		assert_eq!(Some(1), tmp.nb);
		let tmp = col.row.get(1).unwrap();
		assert_eq!(Some(2), tmp.nb);
		let tmp = col.row.get(2).unwrap();
		assert_eq!(Some(3), tmp.nb);
		let tmp = col.row.get(3).unwrap();
		assert_eq!(None, tmp.nb);
		let tmp = col.row.get(4).unwrap();
		assert_eq!(None, tmp.nb);
		let tmp = col.row.get(5).unwrap();
		assert_eq!(None, tmp.nb);
		let tmp = col.row.get(6).unwrap();
		assert_eq!(None, tmp.nb);
		let tmp = col.row.get(7).unwrap();
		assert_eq!(None, tmp.nb);
		let tmp = col.row.get(8).unwrap();
		assert_eq!(None, tmp.nb);
	}
#[test]
	fn test_sudoku_new() {
		let sudo = Sudoku::new(make_grill_test());
		assert!(sudo.is_ok());
		let sudo2 = sudo.unwrap();
		assert_eq!(3, sudo2.cells[2].nb.unwrap());
		assert_eq!(2, sudo2.cells[2].col);
		assert_eq!(0, sudo2.cells[2].row);
		assert_eq!(0, sudo2.cells[2].square);
	}
}
