extern crate colored;
extern crate glob;
extern crate rand;

use std::env;

use colored::Colorize;
use rand::Rng;

use model::{*, image_class::ImageClass::*, input::Input, neural_node::NeuralNode, weight::Weight};

mod model;

const DATA_SETS: usize = 4;
const LEARNING_ITERATIONS: usize = 15000;

macro_rules! print_separator {
    () => (println!("{:=<1$}", "", Y_IMG_SIZE * 4));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_training_mode = match args.len() {
        2 => {
            args[1].to_ascii_lowercase().eq(&String::from("true"))
        },
        _ => {
            false
        }
    };
    let mut rng = rand::thread_rng();

    let mut nn_0 = NeuralNode::new("nn_0", 6000);
    let mut weights: Vec<Weight> = Vec::new();
    if is_training_mode {
        // Read inputs from res/(0..9) folders
        let mut inputs: Vec<Vec<Input>> = Vec::new();
        for i in 0..DATA_SETS {
            let mut ds_folder_name = String::from("res/");
            ds_folder_name.push_str(&i.to_string());
            ds_folder_name.push_str("/*.png");
            inputs.push(Input::inputs_from_path(&ds_folder_name, &Zero));
        }
        // Update weights
        for i in 0..DATA_SETS {
            let mut weight_name = String::from("w_");
            weight_name.push_str(&i.to_string());
            weights.push(Weight::new(&weight_name));
            for _ in 0..LEARNING_ITERATIONS {
                let mut n = rng.gen_range(0, DATA_SETS);
                match n == i {
                    true => nn_0.process_inc_mul(&inputs[i], &mut weights[i]),
                    false => nn_0.process_dec_mul(&inputs[n], &mut weights[i]),
                }
            }
            // Save weights to .txt files
            let mut file_name = String::from("res/persistence/weight_");
            file_name.push_str(&i.to_string());
            file_name.push_str(".txt");
            weights[i].to_file(&file_name).unwrap_or_default();
        }
    } else {
        // Read saved weights
        for f in get_paths("res/persistence/*.txt").iter() {
            weights.push(Weight::from_file(f));
        }
    }

    for w in weights.iter() {
        w.print_colored();
        print_separator!();
    }

    let test_inputs = Input::inputs_from_path("res/test/*.png", &Zero);
    let mut is_of_class: bool;
    for t in test_inputs.iter() {
        for (i, w) in weights.iter().enumerate() {
            is_of_class = nn_0.process(t, w);
            println!("{} is {} - {}", t.name.green(), i, is_of_class);
        }
        print_separator!();
    }
}