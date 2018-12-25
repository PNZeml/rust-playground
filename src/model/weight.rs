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
            coeffs: vec![0; IMAGE_SIZE * IMAGE_SIZE],
            adjust_iteration: 0,
        }
    }

    pub fn update(&mut self, input: &Input, inc: i8) {
        for i in 0..self.coeffs.len() {
            self.coeffs[i] += (input.signals[i] * inc) as i64;
        }
        self.adjust_iteration += 1;
    }

    pub fn print(self) {
        let mut ln_br_cn = 0;
        let mut ln_to_output = String::new();
        for c in self.coeffs {
            ln_to_output.push_str(&c.to_string());
            ln_to_output.push('\t');
            if ln_br_cn == IMAGE_SIZE {
                ln_br_cn = 0;
                println!("{}", ln_to_output);
                ln_to_output = String::new();
            } else {
                ln_br_cn += 1;
            }
        }
    }

    pub fn print_colored(&self) {
        let mut ln_br_cn = 0;
        let mut ln_to_output = String::new();
        for c in self.coeffs.iter() {
            if c > &0i64 {
                print!("{}\t", c.to_string().green())
            } else {
                print!("{}\t", c.to_string().red())
            }
            if ln_br_cn == IMAGE_SIZE - 1{
                ln_br_cn = 0;
                println!();
                ln_to_output = String::new();
            } else {
                ln_br_cn += 1;
            }
        }
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