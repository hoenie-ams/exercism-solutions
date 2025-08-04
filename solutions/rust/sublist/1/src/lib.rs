#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        Comparison::Equal
    } else if first_list.is_empty() {
        Comparison::Sublist
    } else if second_list.is_empty() {
        Comparison::Superlist
    } else if first_list.len() == second_list.len() && first_list == second_list {
        Comparison::Equal
    } else if first_list.len() < second_list.len() {
        return if second_list
            .windows(first_list.len())
            .any(|window| window == first_list)
        {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        };
    } else if first_list.len() > second_list.len() {
        return if first_list
            .windows(second_list.len())
            .any(|window| window == second_list)
        {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        };
    } else {
        Comparison::Unequal
    }
}
