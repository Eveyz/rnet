use std::collections::{HashMap, HashSet};

macro_rules! my_macro {
	() => {
		println!("this is my macro")
	};
}

pub struct Node {
	val: i32,
	name: String
}

pub struct Edge {
	src: Node,
	dest: Node,
	weight: i32
}

pub struct Graph<'a> {
	n: i32,
	edges: HashMap<i32, Vec<&'a Node>>
}

impl<'a> Graph<'a> {
	pub fn new(n: i32) -> Graph<'a> {
		Graph {
			n,
			edges: HashMap::new()
		}
	}

	pub fn add_edge(&'a mut self, src: &'a Node, dest: &'a Node) {
		match self.edges.get_mut(&src.val) {
			Some(neighbors) => {
				neighbors.push(dest);
			},
			None => {
				self.edges.insert(src.val, vec![dest]);
			}
		}
	}

	pub fn dfs(&mut self) {
		
	}
}