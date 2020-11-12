use std::hash::{BuildHasher};
use std::collections::hash_map::RandomState;

const INITIAL_NBUCKETS: usize = 1;
struct Bucket<K, V> {
	items: Vec<(K, V)>
}

pub struct HashMap<K, V, S = RandomState> {
	buckets: Vec<Bucket<K, V>>,
	build_hasher: S,
}

impl<K, V> HashMap<K, V, RandomState> {
	pub fn new() -> Self {
		HashMap {
			buckets: Vec::new(),
			build_hasher: RandomState::new(),
		}
	}
}

impl<K, V, S> HashMap<K, V, S> 
where 
	S: BuildHasher,
{
	fn resize(&mut self) {
		let target_size = match self.buckets.len() {
			0 => INITIAL_NBUCKETS,
			n => 2 * n,
		};
	}

	pub fn insert(&mut self, key: K, value: V) -> Option<V> {
		key.hash() % self.buckets.len()
	}
}