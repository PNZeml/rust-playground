use std::thread::{sleep, spawn};
use std::time::Duration;

use rand::{Rng, thread_rng};
use super::rayon::prelude::*;

use model::{*, image_class::ImageClass::*, input::Input, weight::Weight};

pub struct NeuralNode {
    pub name: String,
    pub bias: i128,
    pub last_output_signal: i128,
}

impl NeuralNode {
    pub fn new(name: &str, bias: i128) -> NeuralNode {
        NeuralNode {
            name: String::from(name),
            bias,
            last_output_signal: 0,
        }
    }

    pub fn process_inc_mul(&mut self, inputs: &Vec<Input>, weight: &mut Weight) {
        for i in inputs.iter() {
            if !self.process(i, weight) {
                weight.update(i, 1);
            }
        }
    }

    pub fn process_dec_mul(&mut self, inputs: &Vec<Input>, weight: &mut Weight) {
        for i in inputs.iter() {
            if self.process(i, weight) {
                weight.update(i, -1);
            }
        }
    }

    pub fn process(&mut self, input: &Input, weight: &Weight) -> bool {
        let res: i128 = input.signals
            .par_iter()
            .zip(weight.coefficients.par_iter())
            .map(|(i, w)| *i * *w)
            .sum();
        self.last_output_signal = res;
        res >= self.bias
    }
}