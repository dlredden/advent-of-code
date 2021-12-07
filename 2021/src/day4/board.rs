use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Board {
	pub number: usize,
	pub rows: Vec<Vec<Number>>,
	pub winning_number: Cell<isize>,
}

impl Board {
	pub fn new(number: usize) -> Board {
		Board { number, rows: Vec::with_capacity(5), winning_number: Cell::new(-1) }
	}

	// Look for the number on the board
	// If found, mark it and stop looking
	// Check for a winning board
	// If found run calculation and return value
	pub async fn mark(&self, number: usize) -> isize {
		if self.winning_number.get().is_positive() {
			return -1
		}
		'outer: for row in &self.rows {
			for num in row {
				if num.number == number {
					num.marked.set(true);
					break 'outer;
				}
			}
		}
		let (winner, sum) = self.is_winner();
		if winner {
			self.winning_number.set((sum * number) as isize);
			return self.winning_number.get();
		}
		return -1;
	}

	fn check_rows(&self) -> bool {
		'outer: for row in 0..5 {
			for num in self.rows[row].iter() {
				if !num.marked.get() {
					continue 'outer
				}
			}
			return true
		}	
		return false
	}

	fn check_cols(&self) -> bool {
		'outer: for col in 0..5 {
			for row in self.rows.iter() {
				if !row[col].marked.get() {
					continue 'outer
				}
			}
			return true
		}	
		return false
	}

	fn sum_unmarked(&self) -> usize {
		let mut sum = 0;
		for row in self.rows.iter() {
			for col in row.iter() {
				if !col.marked.get() {
					sum += col.number
				}	
			}
		}
		return sum;
	}

	// Look through the entire 5x5 grid and look
	// for rows or columns that are fully marked.
	fn is_winner(&self) -> (bool, usize) {
		let row_win = self.check_rows();
		if row_win {
			return (true, self.sum_unmarked())
		}

		let col_win = self.check_cols();
		if col_win {
			// calculate winning numbers
			return (true, self.sum_unmarked())
		}

		return (false, 0)
	}
}

#[derive(Debug, Clone)]
pub struct Number {
	pub number: usize,
	pub marked: Cell<bool>,
}

impl Number {
	pub fn new(number: usize) -> Number {
		Number { number: number, marked: Cell::new(false) }
	}
}