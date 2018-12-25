extern crate rayon;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread::{sleep, spawn};
use std::time::Duration;

use rand::{Rng, thread_rng};
use rayon::prelude::*;

use model::{*, image_class::ImageClass::*, input::Input, weight::Weight};

pub struct NeuralNode {
    pub name: String,
    pub bias: i64,
    pub last_output_signal: i64,
}

impl NeuralNode {
    pub fn new(name: &str, bias: i64) -> NeuralNode {
        NeuralNode {
            name: String::from(name),
            bias,
            last_output_signal: 0,
        }
    }

    pub fn process(&self, input: &Input, weight: &Weight) -> bool {
        let res: i64 = input.signals
            .par_iter()
            .zip(weight.coeffs.par_iter())
            .map(|(i, w)| *i as i64 * *w)
            .sum();
        res >= self.bias
    }
}