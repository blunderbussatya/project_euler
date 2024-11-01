use std::cmp::max;

use utils::sieve;

const MX: usize = 1000000;

fn main() {
    let (primes, is_prime) = sieve::primes(MX);

    let mut len = 0;
    let mut ans = 0;

    for i in 0..primes.len() {
        let mut sum = 0;
        for j in i..primes.len() {
            sum += primes[j];
            if sum > MX {
                break;
            }
            let cur_len = j - i + 1;
            if is_prime[sum] && len < cur_len{
                (len, ans) = (cur_len, sum);
            }
        }
    }
    println!("{ans}");
}
