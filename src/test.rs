use sudoku::Sudoku;
use cell::Cell;
use column::Column;
use square::Square;
use row::Row;
use ROW_SIZE;
use get_square;
const GRILL: &[Option<u8>] = &[
	Some(1), Some(2), Some(3), None, None, None, None, None, None,
	Some(8), Some(7), Some(6), None, None, None, None, None, Some(1),
	None, Some(9), Some(4), Some(8), None, None, None, None, None,
	None, Some(6), None, None, Some(7), None, None, None, None,
	None, None, None, None, None, Some(8), None, None, None,
	None, None, None, Some(3), Some(4), None, None, None, None,
	None, None, None, None, None, None, Some(1), Some(7), Some(9),
	None, None, Some(5), None, None, None, Some(6), None, None,
	None, None, None, Some(6), Some(2), Some(1), None, None, None
];


#[cfg(test)]
mod tests {
	use super::*;

#[test]
	fn test_square_1() {
		assert_eq!(0, get_square(0, 0));
	}
#[test]
	fn test_square_2() {
		assert_eq!(1, get_square(3, 1));
	}
#[test]
	fn test_square_3() {
		assert_eq!(2, get_square(6, 2));
	}
#[test]
	fn test_square_4() {
		assert_eq!(3, get_square(2, 4));
	}
#[test]
	fn test_square_5() {
		assert_eq!(4, get_square(5, 5));
	}
#[test]
	fn test_square_6() {
		assert_eq!(5, get_square(7, 5));
	}
#[test]
	fn test_square_7() {
		assert_eq!(6, get_square(0, 7));
	}
#[test]
	fn test_square_8() {
		assert_eq!(7, get_square(4, 8));
	}
#[test]
	fn test_square_9() {
		assert_eq!(8, get_square(6, 6));
	}
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
#[test]
	fn test_col_new() {
		let sudo = Sudoku::new(GRILL);
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
		let sudo = Sudoku::new(GRILL);
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
#[test]
	fn test_square_new() {
		let sudo = Sudoku::new(GRILL);
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
#[test]
	fn test_row_new() {
		let sudo = Sudoku::new(GRILL);
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
		let sudo = Sudoku::new(GRILL);
		assert!(sudo.is_ok());
		/*match sudo {
		  Err(SudokuErr::GrillSize) =	> assert_eq!(1, 2),
		  Err(SudokuErr::MultResult) => assert_eq!(3, 4),
		  Ok(sud) => {
		  assert_eq!(3, sud.cells[2].nb.unwrap());
		  assert_eq!(2, sud.cells[2].col);
		  assert_eq!(0, sud.cells[2].row);
		  assert_eq!(0, sud.cells[2].square);
		  },
		  }*/
		let sudo2 = sudo.unwrap();
		assert_eq!(3, sudo2.cells[2].nb.unwrap());
		assert_eq!(2, sudo2.cells[2].col);
		assert_eq!(0, sudo2.cells[2].row);
		assert_eq!(0, sudo2.cells[2].square);
	}
}
