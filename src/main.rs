extern crate colored;
extern crate glob;
extern crate rand;
extern crate rayon;
extern crate core;

use std::env;
use std::fs::{self, DirBuilder};

use colored::Colorize;
use rand::Rng;

use model::{*, image_class::ImageClass::*, input::Input, neural_node::NeuralNode, weight::Weight};
use rayon::prelude::*;
use std::path::PathBuf;

mod model;

const DATA_SETS: usize = 6;
const LEARNING_ITERATIONS: usize = 30000;

macro_rules! print_separator {
    () => (println!("{:=<1$}", "", Y_IMG_SIZE * 4));
}

fn create_folder(path: &PathBuf) -> Result<bool, &str> {
    match DirBuilder::new()
        .recursive(true)
        .create(path) {
        Ok(x) => Ok(true),
        Err(_) => Err("Error :\t Cannot create a folder"),
    }
}

fn init_dirs() {
    let mut root_path = std::env::current_exe().unwrap();
    root_path.push("/res");
    for i in 0..=9 {
        let mut new_folder_path  = root_path.clone();
        new_folder_path.push(i.to_string());
        create_folder(&new_folder_path).unwrap_or_default();
    }
    let mut new_folder = root_path.clone();
    new_folder.push("persistence");
    create_folder(&new_folder);
    new_folder = root_path.clone();
    new_folder.push("test");
    create_folder(&new_folder);
}

fn main() {
    init_dirs();
    let args: Vec<String> = env::args().collect();
    let is_training_mode = match args.len() {
        2 => {
            args[1].to_ascii_lowercase().eq(&String::from("true"))
        }
        _ => {
            false
        }
    };
    let mut rng = rand::thread_rng();
    let mut root_path = std::env::current_exe().unwrap();
    root_path.push("/res");

    let mut nn = NeuralNode::new("nn_0", 8000);
    let mut weights: Vec<Weight> = Vec::new();

    if is_training_mode {
        // Read inputs from res/(0..9) folders
        let mut inputs: Vec<Vec<Input>> = Vec::new();
        for i in 0..DATA_SETS {
            let mut pattern = String::from(root_path.to_str().unwrap());
            pattern.push('/');
            pattern.push_str(&i.to_string());
            pattern.push_str("/*.png");
            inputs.push(Input::inputs_from_pattern(&pattern, &Zero));
        }
        // Update weights
        for i in 0..DATA_SETS {
            let mut weight_name = String::from("w_");
            weight_name.push_str(&i.to_string());
            weights.push(Weight::new(&weight_name));
            for _ in 0..LEARNING_ITERATIONS {
                let mut n = rng.gen_range(0, DATA_SETS);
                match n == i {
                    true => nn.process_inc_mul(&inputs[i], &mut weights[i]),
                    false => nn.process_dec_mul(&inputs[n], &mut weights[i]),
                }
            }
            // Save weights to .txt files
            let mut path_to_weight = String::from(root_path.to_str().unwrap());
            path_to_weight.push_str("/persistence/weight_");
            path_to_weight.push_str(&i.to_string());
            path_to_weight.push_str(".txt");
            println!("{}", path_to_weight);
            weights[i].to_file(&path_to_weight).unwrap_or_default();
        }
    } else {
        // Read saved weights
        let mut pattern =  String::from(root_path.to_str().unwrap());
        pattern.push_str("/persistence/*.txt");
        for f in get_paths(&pattern).iter() {
            weights.push(Weight::from_file(f));
        }
    }

    for w in weights.iter() {
        w.print_colored();
        print_separator!();
    }

    let mut pattern =  String::from(root_path.to_str().unwrap());
    pattern.push_str("/test/*.png");
    let test_inputs = Input::inputs_from_pattern(&pattern, &Zero);
    for t in test_inputs.iter() {
        let classes: Vec<i8> = weights.iter()
            .enumerate()
            .filter(|x| nn.process(t, x.1))
            .map(|x| x.0 as i8)
            .collect();
        println!("Image {} is of {:?} classes", t.name.green(), classes);
        print_separator!();
    }
}