extern crate neuroflow;

use neuroflow::activators::Type::Tanh;
use neuroflow::data::DataSet;
use neuroflow::io;
use neuroflow::FeedForward;

fn main() {
    let architecture = vec![3, 5, 5, 2];
    let mut neural_net = FeedForward::new(&architecture);

    let data: DataSet = DataSet::from_csv("data/test.csv").unwrap();

    const ITERATIONS: i64 = 50_000;
    neural_net
        .activation(Tanh)
        .learning_rate(0.1)
        .momentum(0.15)
        .train(&data, ITERATIONS);

    io::save(&mut neural_net, "models/test.flow").unwrap();
}
