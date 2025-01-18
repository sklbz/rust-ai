use neuroflow::{io, FeedForward};

pub fn test() {
    let mut new_nn: FeedForward = io::load("models/test.flow").unwrap();
}
