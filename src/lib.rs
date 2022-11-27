#![allow(unused)]
use std::ops::Add;
use std::ops::Mul;

fn mul_slice<T: Mul + Mul<Output = T> + Copy + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let len = if a.len() >= b.len() { a.len() } else { b.len() };
    let mut v: Vec<T> = Vec::new();

    for i in 0..len {
        v.push(a[i] * b[i]);
    }
    v
}

fn mul_vec(a: &[i32], b: &[i32]) -> Vec<i32> {
    let len = if a.len() >= b.len() { a.len() } else { b.len() };
    let mut v: Vec<i32> = Vec::new();
    for i in 0..len {
        v.push(a[i] * b[i]);
    }
    v
}

struct Neuron {
    bias: i32,
    weights: Vec<i32>,
}

impl Neuron {
    pub fn new(bias: i32, weights: Vec<i32>) -> Self {
        Self { bias, weights }
    }

    pub fn default() -> Self {
        Self::new(0, Vec::<i32>::new())
    }

    pub fn process(&self, data: &Vec<i32>) -> Vec<i32> {
        mul_vec(&self.weights, data)
            .iter()
            .map(|i| i + self.bias)
            .collect()
    }

    pub fn normalize_weights(&mut self, w: Vec<i32>) {
        self.weights = mul_vec(&self.weights, &w);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(true)
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        assert!(false)
    }
}
