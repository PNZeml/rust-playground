use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::path::PathBuf;

use model::{*, input::Input};

use super::colored::Colorize;

#[derive(Debug)]
pub struct Weight {
    pub name: String,
    pub coefficients: Vec<i128>,
    pub adjusting_iteration: u64,
}

impl Weight {
    pub fn new(name: &str) -> Weight {
        Weight {
            name: String::from(name),
            coefficients: vec![0; get_img_size()],
            adjusting_iteration: 0,
        }
    }

    pub fn to_file(&self, path: &str) -> Result<bool, &str> {
        let mut file = match File::create(path) {
            Ok(x) => x,
            Err(_) => return Err("Error :\t An error occurred while a file was creating"),
        };
        self.coefficients.iter().for_each(|c| {
            let mut buf_str = String::from(c.to_string());
            buf_str.push('\n');
            match file.write_all(buf_str.as_bytes()) {
                Ok(x) => x,
                Err(_) => return (),
            }
        });
        match file.flush() {
            Ok(_) => Ok(true),
            Err(_) => Err("Error :\t An error occurred on a file flush"),
        }
    }

    pub fn from_file(path: &PathBuf) -> Weight {
        let br = BufReader::new(File::open(path).unwrap());
        let coefficients: Result<Vec<i128>, Error> = br.lines()
            .map(|line|
                line.and_then(|v|
                    v.parse().map_err(|e|
                        Error::new(ErrorKind::InvalidData, e)
                    )
                )
            )
            .collect();
        Weight {
            name: String::from(format!("{:?}", path)),
            coefficients: coefficients.unwrap(),
            adjusting_iteration: 0,
        }
    }

    pub fn update(&mut self, input: &Input, inc: i128) {
        for i in 0..self.coefficients.len() {
            self.coefficients[i] += input.signals[i] * inc;
        }
        self.adjusting_iteration += 1;
    }

    pub fn print(self) {
        let mut ln_br_cn = 0;
        let mut buf_str = String::new();
        for c in self.coefficients {
            buf_str.push_str(&c.to_string());
            buf_str.push('\t');
            if ln_br_cn == Y_IMG_SIZE - 1 {
                ln_br_cn = 0;
                println!("{}", buf_str);
                buf_str = String::new();
            } else {
                ln_br_cn += 1;
            }
        }
    }

    pub fn print_colored(&self) {
        let mut ln_br_cn = 0;
        for c in self.coefficients.iter() {
            if c > &0i128 {
                print!("{}\t", c.to_string().green())
            } else {
                print!("{}\t", c.to_string().red())
            }
            if ln_br_cn == Y_IMG_SIZE - 1 {
                ln_br_cn = 0;
                println!();
            } else {
                ln_br_cn += 1;
            }
        }
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} :\t Weight coefficients on {} adjusting: {:?}",
               self.name.green(),
               self.adjusting_iteration,
               self.coefficients)
    }
}