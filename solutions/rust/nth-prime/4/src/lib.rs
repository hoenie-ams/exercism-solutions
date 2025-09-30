pub fn nth(n: u32) -> u32 {
    let n = n as f64;
    let size: usize = (n * n.ln() * 1.5) as usize + 100;

    let mut sieve = vec![true; size];

    (2..size)
        .filter(|&i| {
            if sieve[i] {
                let i_squared = i.saturating_mul(i);
                for multiple in (i_squared..size).step_by(i) {
                    sieve[multiple] = false
                }
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap() as u32
}
