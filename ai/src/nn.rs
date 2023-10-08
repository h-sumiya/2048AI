use rand::prelude::*;
use std::arch::x86_64::*;
use std::mem::transmute;
use std::ops;

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
            weights: [[T8; $in_size]; $out_size],
            biases: [T8; $out_size],
        }

        impl $name {
            pub fn new(rng: &mut ThreadRng) -> Self {
                let mut weights = [[T8::new([0f32; 8]); $in_size]; $out_size];
                let mut biases = [T8::new([0f32; 8]); $out_size];
                for i in 0..$out_size {
                    for j in 0..$in_size {
                        weights[i][j] = T8::random(rng, &MINUS_ONE, &ONE);
                    }
                    biases[i] = T8::random(rng, &MINUS_ONE, &ONE);
                }
                $name { weights, biases }
            }

            pub fn calc(&self, input: &[T8; $in_size]) -> [T8; $out_size] {
                let mut res = [T8::new([0f32; 8]); $out_size];
                for i in 0..$out_size {
                    let res = &mut res[i];
                    for j in 0..$in_size {
                        *res += input[j] * self.weights[i][j];
                    }
                    *res += self.biases[i];
                    *res = unsafe { res.relu() };
                }
                res
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
        }
    };
}

layer!(Layer2_4, 2, 4);
layer!(Layer4_4, 4, 4);
output_layer!(OutputLayer4, 4);
network!(Network={ OutputLayer4 , a:Layer2_4 , b:Layer4_4});

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
}
