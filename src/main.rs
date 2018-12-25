extern crate glob;
extern crate rand;
extern crate rayon;

use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use glob::{glob_with, MatchOptions};
use rayon::prelude::*;

use model::*;
use model::image_class::ImageClass::*;
use model::input::Input;
use model::weight::Weight;
use model::neural_node::NeuralNode;
use rand::Rng;

mod model;

fn get_learning_files(uri: &str) -> Vec<PathBuf> {
    glob_with(uri, &Default::default())
        .unwrap()
        .filter_map(|x| x.ok())
        .collect()
}

fn main() {
    let inputs_0: Vec<Input> = get_learning_files("res/0/*.png").par_iter()
        .map(|x| Input::from_image_path(x, Zero))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let inputs_1: Vec<Input> = get_learning_files("res/1/*.png").par_iter()
        .map(|x| Input::from_image_path(x, One))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let inputs_2: Vec<Input> = get_learning_files("res/2/*.png").par_iter()
        .map(|x| Input::from_image_path(x, Two))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let mut w_0 = Weight::new("w_0");
    let mut rng = rand::thread_rng();
    // Train 0
    let nn = NeuralNode::new("nn_0", 10000);
    for _ in 0..1000 {
        let n = rng.gen_range(0, 3);

        match n == 2 {
            true => {
                for i in inputs_2.iter() {
                    if !nn.process(i, &w_0) {
                        w_0.update(i, 1);
                    }
                }
            },
            false => {
                match n {
                    1 => {
                        for i in inputs_1.iter() {
                            if nn.process(i, &w_0) {
                                w_0.update(i, -1);
                            }
                        }
                    },
                    0 => {
                        for i in inputs_0.iter() {
                            if nn.process(i, &w_0) {
                                w_0.update(i, -1);
                            }
                        }
                    }
                    _ => {},
                }
            }
        }
    }

    w_0.print_colored();
    for i in 0..10 {
        let mut b = nn.process(inputs_0.get(i).unwrap(), &w_0);
        println!("{} - {}", inputs_0.get(i).unwrap().name, b);
        let mut b = nn.process(inputs_1.get(i).unwrap(), &w_0);
        println!("{} - {}", inputs_1.get(i).unwrap().name, b);
        let mut b = nn.process(inputs_2.get(i).unwrap(), &w_0);
        println!("{} - {}", inputs_2.get(i).unwrap().name, b);
    }
}