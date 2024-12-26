#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

impl Comparison {
    pub fn is_equal(&self) -> bool {
        self == &Comparison::Equal
    }
    pub fn is_sublist(&self) -> bool {
        self == &Comparison::Sublist
    }
    pub fn is_superlist(&self) -> bool {
        self == &Comparison::Superlist
    }
    pub fn is_unequal(&self) -> bool {
        self == &Comparison::Unequal
    }
}

pub fn is_sublist<T: PartialEq>(small: &[T], big: &[T]) -> bool {
    small.is_empty() || big.windows(small.len()).any(|window| window == small)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if is_sublist(_first_list, _second_list) {
        Comparison::Sublist
    } else if is_sublist(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
