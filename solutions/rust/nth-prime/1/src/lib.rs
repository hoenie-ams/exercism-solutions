pub fn nth(n: u32) -> u32 {
    let size = (n as f64 * (n as f64).ln() * 1.5) as u32 + 100;

    let mut sieve = vec![true; (size) as usize];

    (2..)
        .filter(|&i| {
            if sieve[i as usize] {
                for multiple in (i * 2..size).step_by(i as usize) {
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
