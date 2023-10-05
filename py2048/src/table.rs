use once_cell::sync::Lazy;

fn u16_to_data(d: u16) -> [i32; 4] {
    let mut data = [0; 4];
    for i in 0..4 {
        data[i] = ((d >> (i * 4)) & 0xf) as i32;
    }
    data
}

fn data_to_u16(data: &[i32; 4]) -> u16 {
    let mut d = 0;
    for i in 0..4 {
        d |= (data[i] as u16) << (i * 4);
    }
    d
}

fn data_to_u64(data: &[i32; 4]) -> u64 {
    let mut d = 0;
    for i in 0..4 {
        d |= (data[i] as u64) << (i * 16);
    }
    d
}

fn calc_line_l(data: &[i32; 4]) -> [i32; 4] {
    let mut res = [0; 4];
    let mut index = 0;
    let mut flag = false;
    for num in data.iter() {
        if *num != 0 {
            if flag && res[index - 1] == *num {
                if res[index - 1] != 15 {
                    res[index - 1] += 1;
                }
                flag = false;
            } else {
                res[index] = *num;
                index += 1;
                flag = true;
            }
        }
    }
    res
}

fn calc_line_r(data: &[i32; 4]) -> [i32; 4] {
    let mut res = [0; 4];
    let mut index = 4;
    let mut flag = false;
    for num in data.iter().rev() {
        if *num != 0 {
            if flag && res[index] == *num {
                if res[index] != 15 {
                    res[index] += 1;
                }
                flag = false;
            } else {
                index -= 1;
                res[index] = *num;
                flag = true;
            }
        }
    }
    res
}

pub struct RowData {
    pub right: u16,
    pub left: u16,
    pub free: usize,
}

impl RowData {
    fn new(data_r: &[i32; 4], data_l: &[i32; 4]) -> Self {
        RowData {
            right: data_to_u16(data_r),
            left: data_to_u16(data_l),
            free: data_r.iter().filter(|&x| *x == 0).count(),
        }
    }
}

pub struct ColData {
    pub up: u64,
    pub down: u64,
    pub free: usize,
}

impl ColData {
    fn new(data_u: &[i32; 4], data_d: &[i32; 4]) -> Self {
        ColData {
            up: data_to_u64(data_u),
            down: data_to_u64(data_d),
            free: data_u.iter().filter(|&x| *x == 0).count(),
        }
    }
}

fn calc_line(data: &[i32; 4]) -> (RowData, ColData) {
    let data_l = calc_line_l(data);
    let data_r = calc_line_r(data);
    (
        RowData::new(&data_r, &data_l),
        ColData::new(&data_l, &data_r),
    )
}

pub static TABLE: Lazy<(Vec<RowData>, Vec<ColData>)> = Lazy::new(|| {
    let mut row_data: Vec<RowData> = Vec::with_capacity(65536);
    let mut col_data: Vec<ColData> = Vec::with_capacity(65536);
    for i in 0..=65535u16 {
        let data = u16_to_data(i);
        let (row, col) = calc_line(&data);
        row_data.push(row);
        col_data.push(col);
    }
    (row_data, col_data)
});
