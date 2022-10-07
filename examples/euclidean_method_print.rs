/*
    打印0~100之间所有数字之间的最大公约数
*/
extern crate comp_number_theory;

use comp_number_theory::base::euclidean_method;
use itertools::Itertools;

fn main() {
    for per_vec in (0..100).permutations(2) {
        println!("GCD of {} and {} is {}.", per_vec[0], per_vec[1], euclidean_method::gcd(per_vec[0], per_vec[1]))
    }
}
