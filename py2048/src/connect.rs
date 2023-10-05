use crate::engine::Board;

impl Board {
    pub fn dump(&self) -> Vec<u8> {
        let mut buf = [0u8; 16];
        buf[0..8].copy_from_slice(&self.data.to_le_bytes());
        buf[8..16].copy_from_slice(&self.seed.to_le_bytes());
        buf.to_vec()
    }
    pub fn load(data: &[u8]) -> Self {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&data[8..16]);
        let seed = u64::from_le_bytes(buf);
        buf.copy_from_slice(&data[0..8]);
        let data = u64::from_le_bytes(buf);
        Board { seed, data }
    }
    pub fn from_vec(data: &[u8], seed: u64) -> Self {
        let mut board = 0u64;
        for i in 0..16 {
            board |= (data[i] as u64) << (i * 4);
        }
        Board { seed, data: board }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for i in 0..16 {
            res.push(((self.data >> (i * 4)) & 15) as u8);
        }
        res
    }
}
