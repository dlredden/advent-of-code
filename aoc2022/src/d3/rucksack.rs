use itertools::Itertools;

pub fn find_common_items (half1: &str, half2: &str) -> Vec<char> {
	let mut common: Vec<char> = Vec::new();
	for c in half1.chars() {
		if half2.find(c) != None {
			common.push(c);
		}
	}

	common.into_iter().unique().collect()
}

pub static PRIORITIES: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";