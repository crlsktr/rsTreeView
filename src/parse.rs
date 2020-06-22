/*
This is specific to the shape of data I have
*/
#[derive(Debug, Clone)]
pub struct Node {
	path: String,
	id: i32,
	depth: i32,
	class_number: i32,
	children: Option<Vec<Node>>,
}

fn place_in_tree(parent: &mut Node, child: Node) {
	// if child.path[last segment] == parent.id then child belongs below parent
	let segmented_str_path: Vec<&str> = child.path
		.split("/")
		.collect();
	let mut segmented_path: Vec<i32> = segmented_str_path.iter()
	.map(|x| x.parse::<i32>().unwrap() )
	.collect();
	let last_child_segment = match segmented_path.pop() {
		Some(value) => value,
		None => 0,
	};
	if last_child_segment == parent.id {
		match &mut parent.children { //Have to borrow as mutable because we're modifying the children array
			Some(arr) => arr.push(child),
			None => {
				parent.children = Some(vec![child]);
			}
		};
	} else {
		let mut segment_iterator = segmented_path.iter();
		segment_iterator.position(|&x| x == parent.id);
		let next_possible_parent = match segment_iterator.next(){
			Some(x) => x,
			None => {
				eprintln!("Parent is not in the path of child. This means that the node {:?} is disconnected from the tree", child);
				return;
			}
		};
		// 	find the current parent in the child path and select the next descendant in the child's path
		// 	call again with the found new parent and new
	}
}

pub fn build_tree(nodes: &mut Vec<Node>) -> Node {
	//sort nodes
	nodes.sort_by(|a, b| a.depth.cmp(&b.depth));
	let mut root = nodes.remove(0);
	while nodes.len() > 0 {
		let node = nodes.remove(0);
		place_in_tree(&mut root, node);
	}
	return root;
}

pub fn parse_nodes(content: &str) -> Vec<Node> {
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
			Err(_e) => -1,
		};
		let class = match segments[3].parse::<i32>() {
			Ok(value) => value,
			Err(_e) => 0,
		};
		let mut suffix = id.to_string().to_owned();
		suffix.push_str("/");
		let trimmed_path = match path.strip_suffix(suffix.as_str()) {
			Some(value) => value,
			None => path,
		};

		if class > 0 && id > 0 {
			nodes.push(Node {
				id: id,
				path: String::from(trimmed_path),
				depth: depth,
				class_number: class,
				children: Some(Vec::new()),
			});
		}
	}
	return nodes;
}
