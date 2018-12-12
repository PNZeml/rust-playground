use std::fmt;
use std::fmt::Display;

#[derive(Clone)]
pub enum ImageClass {
    Positive,
    Negative,
}

impl ImageClass {
    pub fn value(&self) -> i8 {
        match self {
            ImageClass::Positive => 1,
            ImageClass::Negative => -1,
        }
    }
}

impl Display for ImageClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result<> {
        write!(f, "{}", self.value())
    }
}

impl PartialEq for ImageClass {
    fn eq(&self, other: &ImageClass) -> bool {
        self.value() == other.value()
    }
}