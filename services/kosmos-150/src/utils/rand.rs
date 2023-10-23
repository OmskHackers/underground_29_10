extern crate rand;

use rand::Rng;

pub fn get_rand_element<T>(v: &Vec<T>) -> &T {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..v.len());
    &v[random_index]
}