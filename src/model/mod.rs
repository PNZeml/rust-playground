extern crate colored;
extern crate glob;
extern crate image;
extern crate rayon;

use std::path::PathBuf;

use self::glob::glob_with;

pub mod input;
pub mod weight;
pub mod neural_node;
pub mod image_class;

pub const X_IMG_SIZE: usize = 32;
pub const Y_IMG_SIZE: usize = 32;

pub fn get_img_size() -> usize {
    X_IMG_SIZE * Y_IMG_SIZE
}

pub fn get_paths(path: &str) -> Vec<PathBuf> {
    glob_with(path, &Default::default())
        .unwrap()
        .filter_map(|x| x.ok())
        .collect()
}