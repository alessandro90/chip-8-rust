use chip_8_rust::run;
use std::env;

fn main() {
    run(env::args().collect());
}
