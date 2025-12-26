#![no_std]
use itertools::Itertools;
use itertools::zip_eq; // Need the trait for method-style .zip_eq()

pub struct Container<const N: usize> {
    array: [i32; N],
}

impl<const N: usize> Container<N> {
    pub const fn new() -> Self {
        Self { array: [0; N] }
    }

    pub fn from_iter(&mut self, iter: impl IntoIterator<Item = i32>) {
        for (val, dest) in iter.into_iter().zip_eq(self.array.iter_mut()) {
            *dest = val;
        }
    }
}

pub fn test_container() {
    let mut container = Container::<5>::new();
    container.from_iter([1, 2, 3, 4, 5]);
}

pub fn test() {
    let data = [1, 2, 3, 4, 5];
    for (a, b) in zip_eq(&data[..data.len() - 1], &data[1..]) {
        let _c = a + b;
    }
}
