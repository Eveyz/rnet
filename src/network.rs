use crate::layer::Layer;
use std::cmp::Ordering;
#[derive(Debug)]
pub struct Network {
  pub layers: Vec<Layer>,
}

impl Network {
  	pub fn initialize_network(n_inputs: usize, n_hidden: usize, n_outputs: usize) -> Network {
		let mut layers = Vec::new();
		let hidden_layer = Layer::new(n_inputs, n_hidden, true);
		let output_layer = Layer::new(n_inputs, n_outputs, false);
		layers.push(hidden_layer);
		layers.push(output_layer);
		Network { layers }
  	}

	pub fn forward_propagate(&mut self, row: Vec<f64>) -> Vec<f64> {
		let mut inputs = row;
		for layer in self.layers.iter_mut() {
			let mut new_inputs: Vec<f64> = Vec::new();
			for neuron in layer.neurons.iter_mut() {
				let activation = neuron.activate(&inputs);
				let o = neuron.transfer(activation);
				neuron.output = Some(o);
				new_inputs.push(o);
			}
			inputs = new_inputs;
		}
		inputs
	}

	pub fn backward_propagate_error(&mut self, expected: Vec<f64>) {
		let size = self.layers.len();
		for i in (0..size).rev() {
			let mut errors: Vec<f64> = Vec::new();
			if self.layers[i].hidden {
				// hidden layer
				for j in 0..self.layers[i].neurons.len() {
					let mut error = 0.0;
					for neuron in self.layers[i+1].neurons.iter_mut() {
						error += neuron.weights[j] * neuron.delta.unwrap();
					}
					errors.push(error);
				}
			} else {
				// output layer
				for (j, neuron) in self.layers[i].neurons.iter_mut().enumerate() {
					errors.push(expected[j] - neuron.output.unwrap());
				}
			}
			for (j, neuron) in self.layers[i].neurons.iter_mut().enumerate() {
				neuron.delta = Some(errors[j] * neuron.transfer_derivative(neuron.output.unwrap()));
			}
		}
	}

	pub fn update_weights(&mut self, row: Vec<f64>, l_rate: f64) {
		let size: usize = self.layers.len();
		// let l = row.len();
		let mut inputs: Vec<f64> = row;
		for i in 0..size {
			if i != 0 {
				// not first layer
				// ith layer input is the output of prev layer
				inputs = Vec::new();
				for neuron in self.layers[i-1].neurons.iter() {
					inputs.push(neuron.output.unwrap());
				}
			}
			for neuron in self.layers[i].neurons.iter_mut() {
				for (j, _) in inputs.iter().enumerate() {
					neuron.weights[j] += l_rate * neuron.delta.unwrap() * inputs[j];
				}
				// update bias
				let last = neuron.weights.len() - 1;
				neuron.weights[last] += l_rate * neuron.delta.unwrap();
			}
		}
	}

	pub fn train(&mut self, x_train: Vec<Vec<f64>>, l_rate: f64, n_epoch: i32, n_outputs: usize) {
		for epoch in 0..n_epoch {
			let mut sum_error: f64 = 0.0;
			for row in x_train.iter() {
				let outputs = self.forward_propagate((*row).clone());
				let mut expected = vec![0.0; n_outputs];
				expected[row[row.len() - 1] as usize] = 1.0;
				for i in 0..expected.len() {
					sum_error += (expected[i] - outputs[i]) * (expected[i] - outputs[i]);
				}
				self.backward_propagate_error(expected);
				self.update_weights((*row).clone(), l_rate);
			}
			println!(">epoch={}, lrate={}, error={}", epoch, l_rate, sum_error);
		}
	}

	pub fn predict(&mut self, row: Vec<f64>) -> (usize, f64) {
		let output = self.forward_propagate(row);
		let mut m = 0.0;
		let mut index = 0;
		for (idx, o) in output.iter().enumerate() {
			if *o > m {
				m = *o;
				index = idx;
			}
		}
		(index, m)
	}
}