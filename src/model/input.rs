use std::fmt;
use super::image::{GenericImageView, Pixel, Rgb};
use super::colored::*;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use super::glob::{glob_with, MatchOptions};
use super::rayon::prelude::*;
use model::{*, image_class::{ImageClass, ImageClass::*}};
use super::rayon::iter::IntoParallelRefIterator;

#[derive(Clone)]
pub struct Input {
    pub name: String,
    pub signals: Vec<i128>,
    pub class: ImageClass,
}

impl Input {
    pub fn new(name: &str, signals: &[i128], class: ImageClass) -> Option<Input> {
        if signals.len() == get_img_size() as usize {
            Some(Input {
                name: String::from(name),
                signals: signals.to_vec(),
                class,
            })
        } else {
            println!("{}",
                     format!("Error :\t The number of Signals doesn't equal to the size of image").red());
            None
        }
    }

    pub fn inputs_from_path(path: &str, class: &ImageClass) -> Vec<Input> {
        get_learning_files(path).par_iter()
            .map(|x| Input::input_from_path(x, class))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }

    pub fn input_from_path(path: &Path, class: &ImageClass) -> Option<Input> {
        let image = match image::open(path) {
            Ok(x) => x,
            Err(_) => {
                println!("{}",
                         format!("Error :\t Can't open the image {:?}", path).red());
                return None
            }
        };
        let img_size = image.dimensions().0 * image.dimensions().1;
        if img_size != get_img_size() as u32 {
            println!("{}",
                     format!("Error :\t Wrong the size of the image {:?}", path).red());
            return None
        }
        let signals: Vec<i128> = image.pixels().map(|(_, _, p)|
            if p.channels()[0] != 0 { 1 } else { -1 }
        ).collect();
        Some(Input {
            name: format!("{:?}", path.file_name().unwrap()),
            signals,
            class: class.clone()
        })
    }
}

impl Default for Input {
    fn default() -> Input {
        Input {
            name: String::from("image"),
            signals: vec![-1; get_img_size() as usize],
            class: Zero,
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let signal_sum: i128 = self.signals.iter().sum();
        write!(f, "{}\t\t : sum of signals {} of image {}",
               self.name.green(),
               self.class,
               signal_sum)
    }
}

fn get_learning_files(path: &str) -> Vec<PathBuf> {
    glob_with(path, &Default::default())
        .unwrap()
        .filter_map(|x| x.ok())
        .collect()
}