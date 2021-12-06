#[derive(Debug)]
pub struct Board {
	pub number: usize,
	pub rows: Vec<Vec<Number>>,
}

impl Board {
	pub fn new(number: usize) -> Board {
		Board { number, rows: Vec::with_capacity(5) }
	}
}

#[derive(Debug)]
pub struct Number {
	pub number: usize,
	pub marked: bool,
}

impl Number {
	pub fn new(number: usize) -> Number {
		Number { number: number, marked: false }
	}
}