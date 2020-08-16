use ggez::conf::WindowMode;
use ggez::event;
use ggez::ContextBuilder;
use rand::{thread_rng, RngCore};

use noisemap::{FalloffParameters, NoiseParameters};

use crate::viewer::{Color, Colors, WorldViewer};
use crate::world::{World, WorldParameters};

mod noisemap;
mod util;
mod viewer;
mod world;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Realms", "KBanana")
        .window_mode(
            WindowMode::default()
                .dimensions(1000.0, 1000.0)
                .resizable(true),
        )
        .build()
        .expect("could not create ggez context!");

    let parameters = WorldParameters {
        width: 2000,
        height: 2000,
        water_level: 0.0,
        elevation_parameters: NoiseParameters {
            scale: 0.2,
            octaves: 8,
            persistence: 0.35,
            lacunarity: 3.5,
        },
        falloff: Some(FalloffParameters {
            a: 2.0,
            b: 6.0,
            multiplier: 0.7,
        }),
    };

    let world = World::new(thread_rng().next_u64(), parameters);

    println!("World generated");

    let mut viewer = WorldViewer::new(
        &world,
        Colors {
            sea_low: Color::rgb(35, 45, 84),
            sea_high: Color::rgb(51, 98, 153),
            land_low: Color::rgb(88, 126, 92),
            land_high: Color::rgb(208, 200, 200),
        },
    );
    viewer.update_buffer();

    match event::run(&mut ctx, &mut event_loop, &mut viewer) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
