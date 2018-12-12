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

    pub fn multiple_output_signal_calc(&self, inputs: Vec<Input>) {
        let (sender, receiver): (Sender<i8>, Receiver<i8>) = mpsc::channel();
        let mut thread_handlers = Vec::new();
        let inputs_len = inputs.len();

        for input in inputs {
            let sender = sender.clone();
            let secs_to_sleep = thread_rng().gen_range(0, 10);
            let join_handler = spawn(move || {
                sleep(Duration::from_secs(secs_to_sleep));
                println!("{}\t\t:: slept for {} s.",
                         format!("Thread #{} of {}", thread_id::get(), input.name).red().bold(),
                         secs_to_sleep);
                let sum_of_signals = input.signals
                    .iter()
                    .map(|&x: &i8| if x > 0 { x } else { 0 })
                    .sum();
                sender.send(sum_of_signals).unwrap();
            });

            thread_handlers.push(join_handler);
        }

        for thread_handler in thread_handlers {
            match  thread_handler.join() {
                Ok(_) => {},
                Err(_) => {}
            }
        }

        let mut sum_results = Vec::with_capacity(inputs_len);
        for _ in 0..inputs_len as i8 {
            sum_results.push(receiver.recv());
        }

        let mut res = 0;
        for s in sum_results {
            match s {
                Ok(x) => res += x as i64,
                Err(_) => println!("Something went wrong..."),
            }
        }
        println!("Result is {}", res);
    }
}