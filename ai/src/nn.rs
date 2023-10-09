use rand::prelude::*;
use std::arch::x86_64::*;
use std::mem::transmute;
use std::ops;

use crate::configs::{MUTATION_RANGE, MUTATION_RATE};

const ZERO: __m256 = unsafe { transmute([0f32; 8]) };

#[derive(Debug, Clone, Copy)]
pub struct T8(__m256);

impl T8 {
    pub const fn new(v: [f32; 8]) -> Self {
        T8(unsafe { transmute(v) })
    }

    #[allow(dead_code)]
    #[target_feature(enable = "avx2")]
    pub unsafe fn init(v: f32) -> Self {
        T8(_mm256_set1_ps(v))
    }

    pub fn random(rng: &mut ThreadRng, min: &Self, max: &Self) -> Self {
        let mut v = [0f32; 8];
        for i in 0..8 {
            v[i] = rng.gen();
        }
        let mut res = T8::new(v);
        res *= *max - *min;
        res += *min;
        res
    }

    pub fn mutate(&self, rng: &mut ThreadRng) -> Self {
        let mut v: [f32; 8] = unsafe { transmute(self.0) };
        for i in 0..8 {
            if rng.gen::<f32>() < MUTATION_RATE {
                v[i] += rng.gen::<f32>() * MUTATION_RANGE * 2.0 - MUTATION_RANGE;
            }
        }
        T8::new(v)
    }

    pub fn cross(&self, other: &Self, rng: &mut ThreadRng) -> (Self, Self) {
        let mut v1: [f32; 8] = unsafe { transmute(self.0) };
        let mut v2: [f32; 8] = unsafe { transmute(other.0) };
        for i in 0..8 {
            if rng.gen() {
                let tmp = v1[i];
                v1[i] = v2[i];
                v2[i] = tmp;
            }
        }
        (T8::new(v1), T8::new(v2))
    }

    #[target_feature(enable = "avx2")]
    unsafe fn sum(&self) -> f32 {
        let low = _mm256_castps256_ps128(self.0);
        let high = _mm256_extractf128_ps(self.0, 1);
        let sum = _mm_add_ps(low, high);
        let sum = _mm_hadd_ps(sum, sum);
        let sum = _mm_hadd_ps(sum, sum);
        _mm_cvtss_f32(sum)
    }

    #[target_feature(enable = "avx2")]
    unsafe fn relu(&self) -> Self {
        T8(_mm256_max_ps(self.0, ZERO))
    }

    #[target_feature(enable = "avx2")]
    unsafe fn _add(&self, other: &Self) -> Self {
        T8(unsafe { _mm256_add_ps(self.0, other.0) })
    }

    #[target_feature(enable = "avx2")]
    unsafe fn _add_assign(&mut self, other: &Self) {
        self.0 = _mm256_add_ps(self.0, other.0);
    }

    #[target_feature(enable = "avx2")]
    unsafe fn _sub(&self, other: &Self) -> Self {
        T8(unsafe { _mm256_sub_ps(self.0, other.0) })
    }

    #[target_feature(enable = "avx2")]
    unsafe fn _sub_assign(&mut self, other: &Self) {
        self.0 = _mm256_sub_ps(self.0, other.0);
    }

    #[target_feature(enable = "avx2")]
    unsafe fn _mul(&self, other: &Self) -> Self {
        T8(unsafe { _mm256_mul_ps(self.0, other.0) })
    }

    #[target_feature(enable = "avx2")]
    unsafe fn _mul_assign(&mut self, other: &Self) {
        self.0 = _mm256_mul_ps(self.0, other.0);
    }

    #[allow(dead_code)]
    #[target_feature(enable = "avx2")]
    unsafe fn calc(&self, mul: &Self, add: &Self) -> Self {
        Self(_mm256_fmadd_ps(self.0, mul.0, add.0))
    }

    pub fn dump(&self) -> [f32; 8] {
        unsafe { transmute(self.0) }
    }
}

impl ops::Add for T8 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        unsafe { self._add(&other) }
    }
}

impl ops::AddAssign for T8 {
    fn add_assign(&mut self, other: Self) {
        unsafe { self._add_assign(&other) }
    }
}

impl ops::Sub for T8 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        unsafe { self._sub(&other) }
    }
}

impl ops::SubAssign for T8 {
    fn sub_assign(&mut self, other: Self) {
        unsafe { self._sub_assign(&other) }
    }
}

impl ops::Mul for T8 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        unsafe { self._mul(&other) }
    }
}

impl ops::MulAssign for T8 {
    fn mul_assign(&mut self, other: Self) {
        unsafe { self._mul_assign(&other) }
    }
}

const ONE: T8 = T8::new([1f32; 8]);
const MINUS_ONE: T8 = T8::new([-1f32; 8]);

macro_rules! layer {
    ($name:ident, $in_size:expr, $out_size:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            weights: [[T8; $in_size]; $out_size * 8],
            biases: [T8; $out_size],
        }

        impl $name {
            pub fn new(rng: &mut ThreadRng) -> Self {
                let mut weights = [[T8::new([0f32; 8]); $in_size]; $out_size * 8];
                let mut biases = [T8::new([0f32; 8]); $out_size];
                for i in 0..$out_size {
                    for j in 0..8 {
                        for k in 0..$in_size {
                            weights[i * 8 + j][k] = T8::random(rng, &MINUS_ONE, &ONE);
                        }
                    }
                    biases[i] = T8::random(rng, &MINUS_ONE, &ONE);
                }
                $name { weights, biases }
            }

            pub fn calc(&self, input: &[T8; $in_size]) -> [T8; $out_size] {
                let mut res = [T8::new([0f32; 8]); $out_size];
                for i in 0..$out_size {
                    let mut v = [0f32; 8];
                    for j in 0..8 {
                        let mut sum = T8(ZERO);
                        let w = self.weights[i * 8 + j];
                        for k in 0..$in_size {
                            sum += input[k] * w[k];
                        }
                        v[j] = unsafe { sum.sum() };
                    }
                    res[i] = unsafe { (T8::new(v) + self.biases[i]).relu() };
                }
                res
            }

            pub const fn size() -> usize {
                $in_size * $out_size * 8 * 8 + $out_size * 8
            }

            pub fn dump(&self) -> Vec<f32> {
                let mut res = Vec::with_capacity(Self::size());
                for ws in self.weights.iter() {
                    for w in ws.iter() {
                        res.extend_from_slice(&w.dump());
                    }
                }
                for b in self.biases.iter() {
                    res.extend_from_slice(&b.dump());
                }
                res
            }

            pub fn load(data: &[f32]) -> Self {
                let mut weights = [[T8::new([0f32; 8]); $in_size]; $out_size * 8];
                let mut biases = [T8::new([0f32; 8]); $out_size];
                let mut index = 0;
                for i in 0..$out_size * 8 {
                    for j in 0..$in_size {
                        let mut v = [0f32; 8];
                        for k in 0..8 {
                            v[k] = data[index];
                            index += 1;
                        }
                        weights[i][j] = T8::new(v);
                    }
                }
                for i in 0..$out_size {
                    let mut v = [0f32; 8];
                    for j in 0..8 {
                        v[j] = data[index];
                        index += 1;
                    }
                    biases[i] = T8::new(v);
                }
                $name { weights, biases }
            }

            pub fn mutate(&mut self, rng: &mut ThreadRng) {
                for ws in self.weights.iter_mut() {
                    for w in ws.iter_mut() {
                        *w = w.mutate(rng);
                    }
                }
                for b in self.biases.iter_mut() {
                    *b = b.mutate(rng);
                }
            }

            pub fn cross(&self, other: &Self, rng: &mut ThreadRng) -> (Self, Self) {
                let mut weights = [[T8::new([0f32; 8]); $in_size]; $out_size * 8];
                let mut biases = [T8::new([0f32; 8]); $out_size];
                let mut weights2 = [[T8::new([0f32; 8]); $in_size]; $out_size * 8];
                let mut biases2 = [T8::new([0f32; 8]); $out_size];
                for i in 0..$out_size {
                    for j in 0..8 {
                        for k in 0..8 {
                            let (w1, w2) =
                                self.weights[i * 8 + j][k].cross(&other.weights[i * 8 + j][k], rng);
                            weights[i * 8 + j][k] = w1;
                            weights2[i * 8 + j][k] = w2;
                        }
                    }
                    let (b1, b2) = self.biases[i].cross(&other.biases[i], rng);
                    biases[i] = b1;
                    biases2[i] = b2;
                }
                (
                    $name { weights, biases },
                    $name {
                        weights: weights2,
                        biases: biases2,
                    },
                )
            }
        }
    };
}

macro_rules! output_layer {
    ($name:ident,$in_size:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            weights: [T8; $in_size],
        }

        impl $name {
            pub fn new(rng: &mut ThreadRng) -> Self {
                let mut weights = [T8::new([0f32; 8]); $in_size];
                for i in 0..$in_size {
                    weights[i] = T8::random(rng, &MINUS_ONE, &ONE);
                }
                $name { weights }
            }

            pub fn calc(&self, input: &[T8; $in_size]) -> f32 {
                let mut res = T8::new([0f32; 8]);
                for i in 0..$in_size {
                    res += input[i] * self.weights[i];
                }
                unsafe { res.sum() }
            }

            pub const fn size() -> usize {
                $in_size * 8
            }

            pub fn dump(&self) -> Vec<f32> {
                let mut res = Vec::with_capacity(Self::size());
                for i in 0..$in_size {
                    res.extend_from_slice(&self.weights[i].dump());
                }
                res
            }

            pub fn load(data: &[f32]) -> Self {
                let mut weights = [T8::new([0f32; 8]); $in_size];
                let mut index = 0;
                for i in 0..$in_size {
                    let mut v = [0f32; 8];
                    for k in 0..8 {
                        v[k] = data[index];
                        index += 1;
                    }
                    weights[i] = T8::new(v);
                }
                $name { weights }
            }

            pub fn mutate(&mut self, rng: &mut ThreadRng) {
                for i in 0..$in_size {
                    self.weights[i] = self.weights[i].mutate(rng);
                }
            }

            pub fn cross(&self, other: &Self, rng: &mut ThreadRng) -> (Self, Self) {
                let mut weights = [T8::new([0f32; 8]); $in_size];
                let mut weights2 = [T8::new([0f32; 8]); $in_size];
                for i in 0..$in_size {
                    let (w1, w2) = self.weights[i].cross(&other.weights[i], rng);
                    weights[i] = w1;
                    weights2[i] = w2;
                }
                ($name { weights }, $name { weights: weights2 })
            }
        }
    };
}

macro_rules! network {
    ($name:ident = {$out:ty , $($names:ident : $layer:ty),+}) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            output_layer: $out,
            $($names: $layer),+
        }

        impl $name {
            pub fn new(rng: &mut ThreadRng) -> Self {
                $name {
                    output_layer: <$out>::new(rng),
                    $($names: <$layer>::new(rng)),+
                }
            }

            pub fn calc(&self, input: &[T8; 2]) -> f32 {
                $(let input = &self.$names.calc(input);)+
                self.output_layer.calc(&input)
            }

            pub const fn size() -> usize {
                <$out>::size() $(+ <$layer>::size())+
            }

            pub fn dump(&self) -> Vec<f32> {
                let mut res = Vec::with_capacity(Self::size());
                res.extend_from_slice(&self.output_layer.dump());
                $(res.extend_from_slice(&self.$names.dump());)+
                res
            }

            pub fn load(data: &[f32]) -> Self {
                let mut index = 0;
                let output_layer = <$out>::load(&data[index..]);
                index += <$out>::size();
                $(
                    let $names = <$layer>::load(&data[index..]);
                    index += <$layer>::size();
                )+
                $name {
                    output_layer,
                    $($names),+
                }
            }

            pub fn mutate(&mut self, rng: &mut ThreadRng) {
                $(self.$names.mutate(rng);)+
                self.output_layer.mutate(rng);
            }

            pub fn cross(&self, other: &Self, rng: &mut ThreadRng) -> (Self, Self) {
                let output_layer = self.output_layer.cross(&other.output_layer, rng);
                $(let $names = self.$names.cross(&other.$names, rng);)+
                (
                    $name {
                        output_layer: output_layer.0,
                        $($names: $names.0),+
                    },
                    $name {
                        output_layer: output_layer.1,
                        $($names: $names.1),+
                    }
                )
            }
        }
    };
}

layer!(Layer2_4, 2, 4);
layer!(Layer4_4, 4, 4);
output_layer!(OutputLayer4, 4);
network!(Network={
    OutputLayer4 ,
    a : Layer2_4 ,
    b : Layer4_4
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut rng = thread_rng();
        let network = Network::new(&mut rng);
        let input = [T8::new([1f32; 8]), T8::new([1f32; 8])];
        let output = network.calc(&input);
        println!("{:?}", output);
        println!("{:?}", network);
    }

    #[test]
    fn size() {
        println!("{}", Layer2_4::size());
        println!("{}", Layer4_4::size());
        println!("{}", OutputLayer4::size());
        println!("{}", Network::size());
    }
}
