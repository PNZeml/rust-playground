extern crate colored;
extern crate rand;

use colored::Colorize;
use rand::Rng;

use model::*;
use model::image_class::ImageClass::*;
use model::input::Input;
use model::neural_node::NeuralNode;
use model::weight::Weight;

mod model;

const DATA_SETS: usize = 4;

fn main() {
    let mut rng = rand::thread_rng();

    let mut inputs: Vec<Vec<Input>> = Vec::new();
    inputs.push(Input::inputs_from_path("res/0/*.png", &Zero));
    inputs.push(Input::inputs_from_path("res/1/*.png", &One));
    inputs.push(Input::inputs_from_path("res/2/*.png", &Two));
    inputs.push(Input::inputs_from_path("res/3/*.png", &Three));

    let mut weights: Vec<Weight> = Vec::new();
    let mut nn_0 = NeuralNode::new("nn_0", 6000);

    if !true {
        weights.push(Weight::new("w_0"));
        weights.push(Weight::new("w_1"));
        weights.push(Weight::new("w_2"));
        weights.push(Weight::new("w_3"));

        let mut n: usize = 0;
        for _ in 0..15000 {
            n = rng.gen_range(0, DATA_SETS);

            match n == 0 {
                true => nn_0.process_inc_mul(&inputs[0], &mut weights[0]),
                false => nn_0.process_dec_mul(&inputs[n], &mut weights[0]),
            }
        }
        for _ in 0..15000 {
            n = rng.gen_range(0, DATA_SETS);
            match n == 1 {
                true => nn_0.process_inc_mul(&inputs[1], &mut weights[1]),
                false => nn_0.process_dec_mul(&inputs[n], &mut weights[1]),
            }
        }
        for _ in 0..15000 {
            n = rng.gen_range(0, DATA_SETS);
            match n == 2 {
                true => nn_0.process_inc_mul(&inputs[2], &mut weights[2]),
                false => nn_0.process_dec_mul(&inputs[n], &mut weights[2])
            }
        }
        for _ in 0..15000 {
            n = rng.gen_range(0, DATA_SETS);
            match n == 3 {
                true => nn_0.process_inc_mul(&inputs[3], &mut weights[3]),
                false => nn_0.process_dec_mul(&inputs[n], &mut weights[3])
            }
        }

        weights[0].to_file("res/persistence/weight_0.txt");
        weights[1].to_file("res/persistence/weight_1.txt");
        weights[2].to_file("res/persistence/weight_2.txt");
        weights[3].to_file("res/persistence/weight_3.txt");
    } else {
        weights.push(Weight::from_file("res/persistence/weight_0.txt"));
        weights.push(Weight::from_file("res/persistence/weight_1.txt"));
        weights.push(Weight::from_file("res/persistence/weight_2.txt"));
        weights.push(Weight::from_file("res/persistence/weight_3.txt"));
    }

    weights[0].print_colored();
    println!("{:=<1$}", "", Y_IMG_SIZE * 4);
    weights[1].print_colored();
    println!("{:=<1$}", "", Y_IMG_SIZE * 4);
    weights[2].print_colored();
    println!("{:=<1$}", "", Y_IMG_SIZE * 4);
    weights[3].print_colored();
    println!("{:=<1$}", "", Y_IMG_SIZE * 4);

    let test_inputs = Input::inputs_from_path("res/test/*.png", &One);
    let mut is_of_class = false;
    for t in test_inputs.iter() {
        is_of_class = nn_0.process(t, &weights[0]);
        println!("{} is {} - {}", t.name.green(), Zero, is_of_class);
        is_of_class = nn_0.process(t, &weights[1]);
        println!("{} is {} - {}", t.name.green(), One, is_of_class);
        is_of_class = nn_0.process(t, &weights[2]);
        println!("{} is {} - {}", t.name.green(), Two, is_of_class);
        is_of_class = nn_0.process(t, &weights[3]);
        println!("{} is {} - {}", t.name.green(), Three, is_of_class);
        println!("{:=<1$}", "", Y_IMG_SIZE * 4);
    }
}