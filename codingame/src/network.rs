use std::arch::x86_64::*; //python:del
use std::mem::transmute; //python:del
use std::ops; //python:del

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

macro_rules! layer {
    ($name:ident, $in_size:expr, $out_size:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            weights: [[T8; $in_size]; $out_size * 8],
            biases: [T8; $out_size],
        }

        impl $name {
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
            pub fn calc(&self, input: &[T8; 2]) -> f32 {
                $(let input = &self.$names.calc(input);)+
                self.output_layer.calc(&input)
            }

            pub const fn size() -> usize {
                <$out>::size() $(+ <$layer>::size())+
            }

            pub fn load(data: &[f32]) -> Self {
                let mut index = 0;
                let output_layer = <$out>::load(&data[index..]);
                index += <$out>::size();
                $(
                    let $names = <$layer>::load(&data[index..]);
                    index += <$layer>::size();
                )+
                assert_eq!(index, Self::size());
                $name {
                    output_layer,
                    $($names),+
                }
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

pub static mut NETWORK: Network = unsafe { transmute([0u8; Network::size() * 4]) };
pub fn load_network() {
    let data = [0f32]; //TODO: load from file
    let mut net = Network::load(&data);
    unsafe {
        std::mem::swap(&mut NETWORK, &mut net);
        std::mem::forget(net);
    }
}
