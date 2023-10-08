pub const MUTATION_RATE: f32 = 0.01;
pub const MUTATION_RANGE: f32 = 0.2;

pub const DO_CROSS: usize = 150;
pub const DO_MUTATION: usize = 600;
pub const DO_CHANGE: usize = 100;
pub const NUM_BOTS: usize = DO_CHANGE + DO_CROSS * 2 + DO_MUTATION;
