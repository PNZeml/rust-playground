extern crate colored;
extern crate core;
extern crate glob;
extern crate rand;
extern crate rayon;

use std::env;
use std::fs::DirBuilder;
use std::path::PathBuf;

use colored::Colorize;
use rand::Rng;
use rayon::prelude::*;

use model::{*, image_class::ImageClass::*, input::Input, neural_node::NeuralNode, weight::Weight};

mod model;

const DATA_SETS: usize = 6;
const LEARNING_ITERATIONS: usize = 30000;

macro_rules! print_separator {
    () => (println!("{:=<1$}", "", Y_IMG_SIZE * 4));
}

fn get_res_path() -> Option<PathBuf> {
    let mut path = match std::env::current_exe() {
        Ok(x) => x,
        Err(_) => return None,
    };
    path.push("/res");
    Some(path)
}

fn create_directory(path: &PathBuf) -> Result<bool, &str> {
    match DirBuilder::new()
        .recursive(true)
        .create(path) {
        Ok(_) => Ok(true),
        Err(_) => Err("Error :\t Cannot create a directory"),
    }
}

fn init_dirs() {
    let root_path = get_res_path().unwrap();
    let dirs: Vec<u8> = (0..=11).collect();
    dirs.par_iter().for_each(|x| {
        let mut new_folder_path = root_path.clone();
        match x {
            0..=9 => new_folder_path.push(x.to_string()),
            10 => new_folder_path.push("persistence"),
            11 => new_folder_path.push("test"),
            _ => println!("Error :\t Error while res dirs were createing"),
        }
        // TODO : Handle an error
        create_directory(&new_folder_path);
    });
}

fn main() {
    init_dirs();
    // Get args
    let args: Vec<String> = env::args().collect();
    let is_training_mode = match args.len() {
        2 => args[1].to_ascii_lowercase().eq(&String::from("true")),
        _ => false,
    };

    let mut rng = rand::thread_rng();
    let root_path = get_res_path().unwrap();

    let mut nn = NeuralNode::new("nn_0", 8000);
    let mut weights: Vec<Weight> = Vec::new();

    if is_training_mode {
        // Read inputs from res/(0..9) directories
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
            // TODO : Handle an error
            weights[i].to_file(&path_to_weight);
        }
    } else {
        // Read saved weights
        let mut pattern = String::from(root_path.to_str().unwrap());
        pattern.push_str("/persistence/*.txt");
        for f in get_paths(&pattern).iter() {
            weights.push(Weight::from_file(f));
        }
    }

    for w in weights.iter() {
        w.print_colored();
        print_separator!();
    }

    let mut pattern = String::from(root_path.to_str().unwrap());
    pattern.push_str("/test/*.png");
    let test_inputs = Input::inputs_from_pattern(&pattern, &Zero);
    for t in test_inputs.iter() {
        let classes: Vec<i8> = weights.iter()
            .enumerate()
            .filter(|x| nn.process(t, x.1))
            .map(|x| x.0 as i8)
            .collect();
        println!("{} :\t belongs to {:?} classes", t.name.green(), classes);
        print_separator!();
    }
}