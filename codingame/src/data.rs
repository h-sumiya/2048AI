#[macro_export] // python:del
macro_rules! include_data {
    ($size:expr,$bytes:expr) => {{
        const R: usize = (0xbf - 0x80 + 1);
        const fn f1(v1: u8, v2: u8, v3: u8, fix: bool) -> [u8; 2] {
            let mut res: usize = if fix { 49152 } else { 0 };
            res += R * R * (v1 as usize);
            res += R * (v2 as usize);
            res += v3 as usize;
            [res as u8, (res >> 8) as u8]
        }
        const fn f() -> [u8; $size] {
            let bytes = $bytes.as_bytes();
            let mut i = 0usize;
            let mut v = [0u8; $size];
            let mut j = 0usize;
            while j < $size {
                let h = bytes[i];
                let res = if h == 0xf1 {
                    let v1 = bytes[i + 1] - 0x80;
                    let v2 = bytes[i + 2] - 0x80;
                    let v3 = bytes[i + 3] - 0x80;
                    i += 4;
                    f1(v1, v2, v3, true)
                } else {
                    let v1 = if h <= 0xec { h - 0xe3 } else { h - 0xe4 };
                    let v2 = bytes[i + 1] - 0x80;
                    let v3 = bytes[i + 2] - 0x80;
                    i += 3;
                    f1(v1, v2, v3, false)
                };
                v[j] = res[0];
                if j + 1 < $size {
                    v[j + 1] = res[1];
                }
                j += 2;
            }
            v
        }
        unsafe { std::mem::transmute(f()) }
    }};
}