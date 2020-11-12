use rand::Rng;
#[derive(Debug)]
pub struct Neuron {
  pub weights: Vec<f64>,
  pub output: Option<f64>,
  pub delta: Option<f64>,
}

impl Neuron {
  	pub fn new(n: usize) -> Neuron {
    	let mut weights = Vec::new();
    	for _ in 0..=n {
      		let mut rng = rand::thread_rng();
      		let r: f64 = rng.gen();
      		weights.push(r);
    	}
    	Neuron { weights, output: None, delta: None }
  	}

  	pub fn activate(&self, inputs: &Vec<f64>) -> f64 {
		let mut activation = self.weights[self.weights.len() - 1];
		for i in 0..self.weights.len() - 1 {
			activation += self.weights[i] * inputs[i];
		}
		activation
	}

	pub fn transfer(&self, activation: f64) -> f64 {
		return 1.0 / (1.0 + (-1.0 * activation).exp());
	}

	pub fn transfer_derivative(&self, output: f64) -> f64 {
		return output * (1.0 - output);
	}
}