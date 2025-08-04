#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    use Comparison::*;
    match (first_list.len(), second_list.len()) {
        (0,0) => Equal,
        (_,0) => Superlist,
        (0,_) => Sublist,
        (m,n) if m > n => if first_list.windows(n).any(|window| window == second_list) {Superlist} else {Unequal}
        (m,n) if m < n =>  if second_list.windows(m).any(|window| window == first_list) {Sublist}  else {Unequal}
        (_,_) => if first_list == second_list {Equal} else {Unequal}
    }
}

