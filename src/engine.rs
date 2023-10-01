const MOVE_TABLE: [u16; 65536] = [0; 65536];

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

fn calc_line_l(data: &mut [i32; 4]) -> bool {
    let mut flag = false;
    for i in 0..3 {
        if data[i] == 0 {
            data[i] = data[i + 1];
            data[i + 1] = 0;
            flag = true;
        } else if data[i] == data[i + 1] {
            data[i] += 1;
            data[i + 1] = 0;
            flag = true;
        }
    }
    flag
}

fn calc_line_r(data: &mut [i32; 4]) -> bool {
    let mut flag = false;
    for i in (1..4).rev() {
        if data[i] == 0 {
            data[i] = data[i - 1];
            data[i - 1] = 0;
            flag = true;
        } else if data[i] == data[i - 1] {
            data[i] += 1;
            data[i - 1] = 0;
            flag = true;
        }
    }
    flag
}

struct RowData {
    right: u16,
    left: u16,
    min_r: usize,
    max_r: usize,
    score_r: usize,
    min_l: usize,
    max_l: usize,
    score_l: usize,
}

impl RowData {
    fn new(data_r: &[i32; 4], data_l: &[i32; 4]) -> Self {
        RowData {
            right: data_to_u16(data_r),
            left: data_to_u16(data_l),
            min_r: *data_r.iter().min().unwrap() as usize,
            max_r: *data_r.iter().max().unwrap() as usize,
            score_r: data_r.iter().sum::<i32>() as usize, // TODO: calc score
            min_l: *data_l.iter().min().unwrap() as usize,
            max_l: *data_l.iter().max().unwrap() as usize,
            score_l: data_l.iter().sum::<i32>() as usize, // TODO: calc score
        }
    }
}

struct ColData {
    up: u64,
    down: u64,
}

impl ColData {
    fn new(data_u: &[i32; 4], data_d: &[i32; 4]) -> Self {
        ColData {
            up: data_to_u64(data_u),
            down: data_to_u64(data_d),
        }
    }
}

fn calc_line(mut data: [i32; 4]) -> (RowData, ColData) {
    let data_l = {
        let mut data = data.clone();
        let mut flag = true;
        while flag {
            flag = calc_line_l(&mut data);
        }
        data
    };
    let data_r = {
        let mut flag = true;
        while flag {
            flag = calc_line_r(&mut data);
        }
        data
    };
    (
        RowData::new(&data_r, &data_l),
        ColData::new(&data_r, &data_l),
    )
}

fn calc_table() {
    let mut row_data: Vec<RowData> = Vec::with_capacity(65536);
    let mut col_data: Vec<ColData> = Vec::with_capacity(65536);
    for i in 0..=65535u16 {
        let data = u16_to_data(i);
        let (row, col) = calc_line(data);
        row_data.push(row);
        col_data.push(col);
    }
}

pub fn temp() {
    println!("{}", 0xf);

    let u16_data = data_to_u16(&[0, 15, 2, 1]);
    println!("{}", u16_data);
    let data = u16_to_data(u16_data);
    println!("{:?}", data);
}
