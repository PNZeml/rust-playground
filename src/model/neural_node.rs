use std::time::Duration;
use std::thread::{spawn, sleep};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use model::{image_class::ImageClass::*, input::Input, weight::Weight, *};
use rand::{thread_rng, Rng};

pub struct NeuralNode {
    pub name: String,
    pub last_output_signal: i64,
}

impl NeuralNode {
    pub fn new(name: &str) -> NeuralNode {
        NeuralNode {
            name: String::from(name),
            last_output_signal: 0,
        }
    }

    pub fn output_signal_calc(&mut self, input: &Input, weight: &mut Weight) {
        let mut s = weight.coeffs[0];
        for i in 0..input.signals.len() {
            s += input.signals[i] as i64 * weight.coeffs[i + 1];
        }

        if s > 0 && input.class == Positive || s < 0 && input.class == Negative {
            println!("{}\t:: {} принадлежит классу {} с выходным сигналом {}",
                     self.name.green(),
                     input.name.red().bold(),
                     input.class,
                     s);
            self.last_output_signal = s;
            return;
        } else {
            println!("{}\t:: Не удалось распознать {} с входным классом {}",
                     self.name.green(),
                     input.name.red().bold(),
                     input.class);
            println!("{}\t:: Перерасчет входных коэффицентов...", self.name.green());
            weight.update(input);
            self.output_signal_calc(input, weight)
        }
    }
}