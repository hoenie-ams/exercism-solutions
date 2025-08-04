/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false
    }
    
    code
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .enumerate()
        .try_fold(0u32, | acc, (i, c)|{
            let mut n = c.to_digit(10)?;
            if i % 2 == 1 {
                n *= 2;
                if n > 9 {
                    n -= 9
                }; 
            }
            acc.checked_add(n)
        })
    .map_or(false, |x| x % 10 == 0)
}
