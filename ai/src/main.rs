mod configs;
mod engine;
mod game;
mod nn;
mod world;

use world::World;

fn main() {
    let start = std::time::Instant::now();
    let mut world = World::new();
    loop {
        world.run(8);
        println!("{}: {}", world.generation, world.max());
        world.update();
        if world.generation % 100 == 0 {
            println!("Time: {:?}", start.elapsed());
        }
    }
}
