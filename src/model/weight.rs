use std::fmt;

use model::*;
use model::IMAGE_SIZE;
use model::input::Input;

#[derive(Debug)]
pub struct Weight {
    pub name: String,
    pub coeffs: Vec<i64>,
    pub adjust_iteration: i64,
}

impl Weight {
    pub fn new(name: &str) -> Weight {
        Weight {
            name: String::from(name),
            coeffs: vec![0; IMAGE_SIZE + 1],
            adjust_iteration: 0,
        }
    }

    pub fn new_from_input(name: &str, input: &Input) -> Weight {
        let mut new_weight = Weight::new(name);
        new_weight.update(input);
        new_weight
    }

    pub fn update(&mut self, input: &Input) {
        match self.coeffs.first_mut() {
            Some(x) => *x += input.class.value() as i64,
            None => panic!("An error in {}", self.name),
        }
        for i in 1..self.coeffs.len() {
            self.coeffs[i] += (input.signals[i - 1] * input.class.value()) as i64;
        }
        self.adjust_iteration += 1;
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t\t:: весовые коэффициэнты на {} корректировке: {:?}",
               self.name.green(),
               self.adjust_iteration,
               self.coeffs)
    }
}