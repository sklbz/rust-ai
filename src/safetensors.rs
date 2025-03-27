use neuroflow::{io, FeedForward};

#[allow(dead_code)]
pub fn load() -> () {
    let mut model = io::load::<FeedForward>("models/model-00001-of-000163.safetensors")
        .expect("Failed to load tensor");

    let input = "Hello";

    let output = model.calc(&[0.0]);

    println!("{:#?}", input);
    println!("{:#?}", output);
}
