use std::fmt;

use model::*;
use model::image_class::ImageClass::*;
use model::image_class::ImageClass;

#[derive(Clone)]
pub struct Input {
    pub name: String,
    pub signals: Vec<i8>,
    pub class: ImageClass,
}

impl Input {
    pub fn new(name: &str, signals: &[i8], class_id: ImageClass) -> Option<Input> {
        if signals.len() == IMAGE_SIZE {
            Some(Input {
                name: String::from(name),
                signals: signals.to_vec(),
                class: class_id,
            })
        } else {
            None
        }
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