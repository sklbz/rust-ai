mod test;
mod train;

extern crate neuroflow;

fn main() {
    if !test::test() {
        train::train();
    }
}
