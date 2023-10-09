mod configs;
mod engine;
mod game;
mod nn;
mod world;
mod progress;

use world::World;

fn main() {
    let mut world = World::new();
    loop {
        world.run(8);
        world.update();
    }
}
