use neuroflow::{activators::Type::Tanh, data::DataSet, io, FeedForward};

#[allow(dead_code)]
pub fn train() {
    let architecture = vec![3, 5, 2];
    let mut neural_net = FeedForward::new(&architecture);

    let data: DataSet = DataSet::from_csv("data/test.csv").expect("Unable to load data");

    const ITERATIONS: i64 = 50_000;
    neural_net
        .activation(Tanh)
        .learning_rate(0.1)
        .momentum(0.15)
        .train(&data, ITERATIONS);

    io::save(&mut neural_net, "models/test.flow").expect("Unable to save model");
}
