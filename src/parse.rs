/* 
This is specific to the shape of data I have
*/
#[derive(Debug)]
pub struct Node {
	path: String,
	id: i32,
	depth: i32,
	class_number: i32,
}

pub fn parse_tree(content: &str) -> Vec<Node> {
	let content_split_iterator = content.split("\n");

	let all_lines: Vec<&str> = content_split_iterator.collect();

	let mut nodes: Vec<Node> = Vec::new();

	for line in all_lines {
		//println!("{}",line);
		let segments: Vec<&str> = line.split("\t").collect();

		if segments.len() != 4 {
			eprintln!(
				"skipping because segments are not what we thought {}",
				segments.len()
			);
			continue;
		}

		let path = segments[0];
		let id = match segments[1].parse::<i32>() {
			Ok(value) => value,
			Err(_e) => 0,
		};
		let depth = match segments[2].parse::<i32>() {
			Ok(value) => value,
			Err(_e) => 0,
		};
		let class = match segments[3].parse::<i32>() {
			Ok(value) => value,
			Err(_e) => 0,
		};

		if class > 0 && id > 0 {
			nodes.push(Node {
				id: id,
				path: String::from(path),
				depth: depth,
				class_number: class,
			});
		}
	}	return nodes;
}
