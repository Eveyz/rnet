mod network;
mod neuron;
mod layer;
mod graph;
mod hashmap;
mod config;

use std::cell::RefCell;
use std::thread;
use std::sync::{Mutex, Arc};

fn main() {

    let cfg = config::MLPLayerConfigurationBuilder::builder()
                                            .activation()
                                            .weight_init()
                                            .build();

    let mut network = network::Network::initialize_network();

    // let dataset = [[2.7810836,2.550537003,0.0].to_vec(),
    //                                 [1.465489372,2.362125076,0.0].to_vec(),
    //                                 [3.396561688,4.400293529,0.0].to_vec(),
    //                                 [1.38807019,1.850220317,0.0].to_vec(),
    //                                 [3.06407232,3.005305973,0.0].to_vec(),
    //                                 [7.627531214,2.759262235,1.0].to_vec(),
    //                                 [5.332441248,2.088626775,1.0].to_vec(),
    //                                 [6.922596716,1.77106367,1.0].to_vec(),
    //                                 [8.675418651,-0.242068655,1.0].to_vec(),
    //                                 [7.673756466,3.508563011,1.0].to_vec()].to_vec();

    // let n_inputs: usize = dataset[0].len();
    // let n_ouputs: usize = 2;
    // let mut network = network::Network::initialize_network(n_inputs, 1, n_ouputs);
    // // println!("{:?}", network);
    // network.train(dataset, 0.5, 50, n_ouputs);
    // println!("{:?}", network);

    // let y = [[2.7810836,2.550537003,0.0].to_vec(),
    //                                 [1.465489372,2.362125076,0.0].to_vec(),
    //                                 [3.396561688,4.400293529,0.0].to_vec(),
    //                                 [1.38807019,1.850220317,0.0].to_vec(),
    //                                 [3.06407232,3.005305973,0.0].to_vec(),
    //                                 [7.627531214,2.759262235,1.0].to_vec(),
    //                                 [5.332441248,2.088626775,1.0].to_vec(),
    //                                 [6.922596716,1.77106367,1.0].to_vec(),
    //                                 [8.675418651,-0.242068655,1.0].to_vec(),
    //                                 [7.673756466,3.508563011,1.0].to_vec()].to_vec();

    // for row in y.iter() {
    //     let prediction = network.predict((*row).clone());
    //     println!("Expected {}, Predicted: {:?}", row[row.len()-1], prediction);
    // }
}
