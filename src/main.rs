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

mod model;

fn get_learning_files(uri: &str) -> Vec<PathBuf> {
    glob_with(uri, &Default::default())
        .unwrap()
        .filter_map(|x| x.ok())
        .collect()
}

fn main() {
    let inputs_0: Vec<Input> = get_learning_files("res/0/*.png").par_iter()
        .map(|x| Input::from_image_path(x, Negative))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let inputs_1: Vec<Input> = get_learning_files("res/1/*.png").par_iter()
        .map(|x| Input::from_image_path(x, Positive))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let mut w_0 = Weight::new("w_0");
    for i in inputs_0 {
        w_0.update(&i);
    }

    let mut w_1 = Weight::new("w_1");
    for i in inputs_1 {
        w_1.update(&i);
    }
    w_1.print();
}