mod parse;
use std::fs;
use parse::{parse_nodes,build_tree};

fn main() {
	let filename = std::env::args().nth(1).expect("No file name given");
	let content = fs::read_to_string(filename)
		.expect("Something went wrong trying to read the file");
	let mut result = parse_nodes(&content);
	let tree = build_tree(&mut result);
	println!("root {:#?}", serde_json::to_string(&tree).unwrap());
}
