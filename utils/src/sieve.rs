pub fn primes(n: usize) -> (Vec<usize>, Vec<bool>) {
    let mut is_prime = vec![true; n + 1];
    (0..2).for_each(|i| is_prime[i] = false);
    for i in 2..=n {
        if !is_prime[i] {
            continue;
        }
        ((2 * i)..=n).step_by(i).for_each(|j| is_prime[j] = false);
    }
    ((2..=n).filter(|&i| is_prime[i]).collect(), is_prime)
}
