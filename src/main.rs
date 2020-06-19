mod parse;

use std::fs;
use parse::{parse_tree};

fn main() {
	let filename = std::env::args().nth(1).expect("No file name given");
	let content = fs::read_to_string(filename)
		.expect("Something went wrong trying to read the file");
	let result = parse_tree(&content);
	println!("results length {}",  result.len());
	for node in result {
		println!("node: {:?}", node);
	}
}
