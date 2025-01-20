use neuroflow::{io, FeedForward};

pub fn test() -> bool {
    let mut neural_net: FeedForward = io::load("models/test.flow").expect("Failed to load model");

    let input = [1.0, 3.0, 5.0];

    let output = neural_net.calc(&input);

    println!("input: {input:#?}\t output: {output:#?}");

    true
}
