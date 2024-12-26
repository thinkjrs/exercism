#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_sublist<T: PartialEq>(small: &[T], big: &[T]) -> bool {
    small.is_empty() || big.windows(small.len()).any(|window| window == small)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list, _second_list) {
        _ if _first_list == _second_list => Comparison::Equal,
        _ if is_sublist(_first_list, _second_list) => Comparison::Sublist,
        _ if is_sublist(_second_list, _first_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
