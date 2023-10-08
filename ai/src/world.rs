use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::configs::*;
use crate::{engine::random_seed, game::Game, nn::Network};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct World {
    bots: Arc<Vec<Game>>,
    pub generation: usize,
}

impl World {
    pub fn new() -> Self {
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

    pub fn run(&mut self, workers: usize) {
        random_seed();
        let index = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..workers {
            let bots = self.bots.clone();
            let index = index.clone();
            let handle = thread::spawn(move || loop {
                let index = {
                    let mut index = index.lock().unwrap();
                    let i = *index;
                    if i >= NUM_BOTS {
                        break;
                    }
                    *index += 1;
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
