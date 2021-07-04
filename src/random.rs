use rand::{
    distributions::Standard,
    prelude::{Distribution, ThreadRng},
    Rng,
};
use std::marker::PhantomData;

pub struct Generator<T> {
    rng: ThreadRng,
    phantom_data: PhantomData<T>,
}

impl<T> Generator<T>
where
    Standard: Distribution<T>,
{
    pub fn new() -> Generator<T> {
        Generator {
            rng: rand::thread_rng(),
            phantom_data: PhantomData,
        }
    }

    pub fn get_random(&mut self) -> T {
        self.rng.gen()
    }
}
