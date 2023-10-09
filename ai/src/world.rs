use crate::configs::*;
use crate::progress::Pbar;
use crate::{engine::random_seed, game::Game, nn::Network};
use once_cell::sync::Lazy;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct World {
    pub bots: Arc<Vec<Game>>,
    pub generation: usize,
}

static PATH: Lazy<PathBuf> = Lazy::new(|| {
    println!("Please input the path to save the data: ");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let path = Path::new(buf.trim());
    if !path.parent().unwrap().exists() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    }
    path.to_path_buf()
});

impl World {
    pub fn new() -> Self {
        if PATH.exists() {
            Self::load_from_file()
        } else {
            let mut bots = Vec::with_capacity(NUM_BOTS);
            let mut rng = thread_rng();
            for _ in 0..NUM_BOTS {
                let network = Network::new(&mut rng);
                bots.push(Game::new(network));
            }
            World {
                bots: Arc::new(bots),
                generation: 0,
            }
        }
    }

    pub fn run(&mut self, workers: usize) {
        random_seed();
        let index = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        println!("Running Generation {}...", self.generation);
        let pbar = Pbar::new();
        for _ in 0..workers {
            let bots = self.bots.clone();
            let index = index.clone();
            let pbar = pbar.clone();
            let handle = thread::spawn(move || loop {
                let index = {
                    let mut index = index.lock().unwrap();
                    let i = *index;
                    if i >= NUM_BOTS {
                        break;
                    }
                    *index += 1;
                    pbar.inc(1);
                    i
                };
                unsafe {
                    let bot = bots.get_unchecked(index) as *const Game as *mut Game;
                    (*bot).run_with_ai(4);
                }
            });
            handles.push(handle);
        }
        handles.into_iter().for_each(|h| h.join().unwrap());
        pbar.finish();
        self.log();
    }

    pub fn max(&self) -> usize {
        let mut max = 0;
        let mut index = 0;
        for (i, bot) in self.bots.iter().enumerate() {
            if bot.score > max {
                max = bot.score;
                index = i;
            }
        }
        let mut bot = self.bots[index].clone();
        println!("{}", bot.run_with_ai(4).data);
        max
    }

    pub fn log(&self) {
        println!("Generation{} max score: {}", self.generation, self.max());
    }

    pub fn dump(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(NUM_BOTS * Network::size() + 4);
        for bot in self.bots.iter() {
            let data = bot.network.dump();
            res.extend_from_slice(&data);
        }
        let mut res = unsafe {
            let ptr = res.as_ptr() as *mut u8;
            std::mem::forget(res);
            let len = NUM_BOTS * Network::size() * 4;
            Vec::from_raw_parts(ptr, len, len + 4)
        };
        let generation = self.generation as u32;
        res.extend_from_slice(&generation.to_le_bytes());
        res
    }

    pub fn load(data: &[u8]) -> Self {
        unsafe {
            let ptr = data.as_ptr() as *mut f32;
            let len = data.len() / 4;
            let slice = std::slice::from_raw_parts(ptr, len);
            let mut bots = Vec::with_capacity(NUM_BOTS);
            for i in 0..NUM_BOTS {
                let network = Network::load(&slice[i * Network::size()..]);
                bots.push(Game::new(network));
            }
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&data[NUM_BOTS * Network::size() * 4..]);
            World {
                bots: Arc::new(bots),
                generation: u32::from_le_bytes(buf) as usize,
            }
        }
    }

    pub fn save(&self) {
        let data = self.dump();
        std::fs::write(&*PATH, data).unwrap();
    }

    pub fn load_from_file() -> Self {
        let data = std::fs::read(&*PATH).unwrap();
        World::load(&data)
    }

    pub fn index(&self) -> WeightedIndex<usize> {
        let mut weights = Vec::with_capacity(NUM_BOTS);
        let mut min = 100_000;
        for bot in self.bots.iter() {
            let socre = bot.score;
            weights.push(socre);
            min = min.min(socre);
        }
        min *= 9;
        min /= 10;
        weights.iter_mut().for_each(|w| *w -= min);
        WeightedIndex::new(&weights).unwrap()
    }

    pub fn update(&mut self) {
        let index = self.index();
        let mut rng = thread_rng();
        let mut bots = Vec::with_capacity(NUM_BOTS);
        for _ in 0..DO_MUTATION {
            let i = index.sample(&mut rng);
            let bot = self.bots.get(i).unwrap();
            let mut network = bot.network;
            network.mutate(&mut rng);
            bots.push(Game::new(network));
        }
        for _ in 0..DO_CROSS {
            let i = index.sample(&mut rng);
            let j = index.sample(&mut rng);
            let bot1 = self.bots.get(i).unwrap();
            let bot2 = self.bots.get(j).unwrap();
            let networks = bot1.network.cross(&bot2.network, &mut rng);
            bots.push(Game::new(networks.0));
            bots.push(Game::new(networks.1));
        }
        for _ in 0..DO_CHANGE {
            let i = index.sample(&mut rng);
            let bot = self.bots.get(i).unwrap();
            bots.push(bot.clone());
        }
        self.bots = Arc::new(bots);
        self.generation += 1;
        if self.generation % SAVE_INTERVAL == 0 {
            self.save();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut world = World::new();
        world.run(7);
    }
}
