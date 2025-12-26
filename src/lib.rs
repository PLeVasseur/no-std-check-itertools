#![no_std]
use itertools::zip_eq;

pub fn test() {
    let data = [1, 2, 3, 4, 5];
    for (a, b) in zip_eq(&data[..data.len() - 1], &data[1..]) {
        let _c = a + b;
    }
}
