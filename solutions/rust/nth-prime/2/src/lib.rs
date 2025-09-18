pub fn nth(n: u32) -> u32 {
    let n = n as f64;
    let size = (n * n.ln() * 1.5) as u32 + 100;

    let mut sieve = vec![true; (size) as usize];

    (2..size)
        .filter(|i: &u32| {
            if sieve[*i as usize] {
                let i_squared = i.saturating_mul(*i);
                for multiple in (i_squared..size).step_by(*i as usize) {
                    sieve[multiple as usize] = false
                }
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}
