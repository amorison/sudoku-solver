/// Number of elements in an Iterator, up to some value.
#[derive(Copy, Clone)]
pub enum CounterUpTo {
    /// The length of the iterator is exactly the value.
    Exactly(usize),
    /// The length of the iterator is stricly more than the value.
    MoreThan(usize),
}

pub fn count_saturated<T: Iterator>(it: &mut T, up_to: usize) -> CounterUpTo {
    let count = it.take(up_to).count();
    if count == up_to && it.next().is_some() {
        CounterUpTo::MoreThan(up_to)
    } else {
        CounterUpTo::Exactly(count)
    }
}
