use nalgebra::Matrix;

struct MultiLayerPerceptron {
    architecture: Vec<u32>,
    // Should I reduce all the operations to a single matrix multiplication or not ?
    // Does adding more layers really makes a model more efficient ?
    // A layer can be represented as a matrix A
    // But A'A (the succession of the two layers) is also a matrix
    // Maybe it is just about having enoug weights to parametrize during training?
    // I shall probably keep different matrices before training and collapse them after
    weights: Vec<Matrix<f64, f64, f64, f64>>,
}

impl MultiLayerPerceptron {}
