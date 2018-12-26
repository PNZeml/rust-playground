use std::fmt;
use std::fmt::Display;

#[derive(Clone)]
pub enum ImageClass {
    Zero,
    One,
    Two,
    Three
}

impl ImageClass {
    pub fn value(&self) -> i8 {
        match self {
            ImageClass::Zero => 0,
            ImageClass::One => 1,
            ImageClass::Two => 2,
            ImageClass::Three => 3,
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