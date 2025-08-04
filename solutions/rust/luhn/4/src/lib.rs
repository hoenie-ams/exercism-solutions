/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().filter(|c| {!c.is_whitespace()}).collect::<Vec<char>>().len() <= 1 || (code.chars().any(|c| !c.is_ascii_digit() && !c.is_whitespace())) {
        return false
    }

    let digits: Vec<char> = code.chars().filter(|c| {c.is_ascii_digit()}).collect();
    
    let sum: u32 = digits
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, n)|{
            let number = n.to_digit(10).unwrap();
            let mut to_add = number * (1 + i as u32 % 2);
            if to_add > 9 {
                to_add -= 9;
            }; 
            to_add
        }).sum();
    (sum % 10) == 0
    
}
