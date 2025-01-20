use neuroflow::{io, FeedForward};

pub fn test() -> bool {
    let mut neural_net: FeedForward = io::load("models/test.flow").expect("Failed to load model");

    let input = [11.0, 13.0, 15.0];
    // [1.0, 3.0, 15.0] -> [1, 5]
    // [11.0, 13.0, 15.0] -> [5, 8]

    let output = neural_net.calc(&input);

    println!("input: {input:#?}\t output: {output:#?}");

    true
}
