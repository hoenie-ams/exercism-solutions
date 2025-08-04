/// Compute the Scrabble score for a word.

pub fn score(word: &str) -> u64 {
    //                         a  b  c  d  e  f  g  h  i  j  k  l  m  n  o  p   q  r  s  t  u  v  w  x  y  z
    let alphabet: [u64; 26] = [1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10];
    let mut result = 0;
    for char in word.chars() {
        let c = char.to_ascii_uppercase() as usize;
        if c <= 122 {  // 122 for 'z'
            result += alphabet[c - 65];  // 65 for 'A'
        }
    }
    return result;
}
