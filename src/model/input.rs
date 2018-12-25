extern crate image;

use std::fmt;
use self::image::{GenericImageView, Pixel, Rgb};

use model::*;
use model::image_class::ImageClass;
use model::image_class::ImageClass::*;
use std::path::Path;
use model::IMAGE_SIZE;

#[derive(Clone)]
pub struct Input {
    pub name: String,
    pub signals: Vec<i8>,
    pub class: ImageClass,
}

impl Input {
    pub fn new(name: &str, signals: &[i8], class: ImageClass) -> Option<Input> {
        if signals.len() == IMAGE_SIZE {
            Some(Input {
                name: String::from(name),
                signals: signals.to_vec(),
                class,
            })
        } else {
            None
        }
    }

    pub fn from_image_path(path: &Path, class: ImageClass) -> Option<Input> {
        let image = match image::open(path) {
            Ok(x) => x,
            Err(_) => return None
        };
        let img_size = IMAGE_SIZE as u32 * IMAGE_SIZE as u32;
        if image.dimensions().0 * image.dimensions().1 != img_size {
            return None
        }
        let signals = image.pixels().map(|(_, _, p)| {
            if p.channels()[0] != 0 {
                1
            } else {
                -1
            }
        }).collect();
        Some(Input {
            name: format!("{:?}", path),
            signals,
            class
        })
    }
}

impl Default for Input {
    fn default() -> Input {
        Input {
            name: String::from("image"),
            signals: vec![-1; IMAGE_SIZE],
            class: Positive,
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t\t:: входные сигналы изображения класса {}: {:?}",
               self.name.green(),
               self.class,
               self.signals)
    }
}