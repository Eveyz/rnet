use crate::neuron::Neuron;
#[derive(Debug)]
pub struct Layer {
	pub neurons: Vec<Neuron>,
	pub hidden: bool
}

impl Layer {
	pub fn new(n_inputs: usize, n: usize, hidden: bool) -> Layer {
		let mut neurons = Vec::new();
		if hidden {
			for _ in 0..n {
				let n = Neuron::new(n_inputs);
				neurons.push(n);
			}
		} else {
			for _ in 0..n {
				let n = Neuron::new(1);
				neurons.push(n);
			}
		}
		Layer { neurons, hidden }
	}
}