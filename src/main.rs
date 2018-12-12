extern crate rand;

use model::*;
use model::image_class::ImageClass::*;

mod model;

#[derive(Debug)]
struct Example {
    u_long: u128,
    double: f64,
}

fn main() {
    let mut input1 = input::Input::new(
        "image1",
        &[1, -1, 1, 1, 1, 1, -1, -1, 1],
        Positive,
    ).unwrap();
    let input2 = input::Input::new(
        "image2",
        &[1, 1, 1, 1, -1, 1, 1, -1, 1],
        Negative,
    ).unwrap();

    println!("{}\n{}", input1, input2);

    let mut w = weight::Weight::new_from_input("weight1", &input1);
    w.update(&input2);
    println!("{}", w);

    let mut neural_node1 = neural_node::NeuralNode::new("neuralNode1");
    neural_node1.output_signal_calc(&input1, &mut w);
    neural_node1.output_signal_calc(&input2, &mut w);

    let mut input3 = model::input::Input::new(
        "image3",
        &[-1, 1, 1, -1, 1, 1, -1, -1, -1],
        Negative,
    ).unwrap();

    neural_node1.output_signal_calc(&mut input3, &mut w);
    let mut input4 = model::input::Input {
        name: "image4".to_string(),
        signals: vec![-1, 1, 1, -1, -1, -1, 1, 1, -1],
        class: Positive,
    };

    neural_node1.output_signal_calc(&mut input4, &mut w);
    neural_node1.output_signal_calc(&mut input1, &mut w);

    let mut input1_clone = input1.clone();
    input1_clone.name = String::from("image1' clone");
    let inputs = vec![input1, input2, input3, input4, input1_clone];
    neural_node1.multiple_output_signal_calc(inputs);

    let ex = Example { u_long: 128, double: 128.0 };
    let ref_to_ex = &ex;
    let moved_ex = ex;
    
    println!("{:?}", ref_to_ex);
}