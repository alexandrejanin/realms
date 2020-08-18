#![warn(clippy::all)]

use ggez::conf::WindowSetup;
use ggez::{conf::WindowMode, event, graphics::Font, ContextBuilder, GameResult};
use rand::{thread_rng, RngCore};

use noisemap::{FalloffParameters, NoiseParameters};

use crate::{
    viewer::{Color, Colors, WorldViewer},
    world::{World, WorldParameters},
};

mod noisemap;
mod util;
mod viewer;
mod world;

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Realms", "KBanana")
        .window_mode(
            WindowMode::default()
                .dimensions(600.0, 600.0)
                .resizable(true),
        )
        .window_setup(WindowSetup::default().title("Realms"))
        .build()
        .expect("could not create ggez context!");

    let world = World::new(
        thread_rng().next_u64(),
        WorldParameters {
            width: 500,
            height: 500,
            sea_level: 0.0,
            elevation_parameters: NoiseParameters {
                scale: 0.2,
                octaves: 8,
                persistence: 0.35,
                lacunarity: 4.0,
            },
            falloff: Some(FalloffParameters {
                a: 2.0,
                b: 6.0,
                multiplier: 0.7,
            }),
        },
    );

    println!("World generated");

    let font = Font::new_glyph_font_bytes(&mut ctx, include_bytes!("Px437_IBM_VGA_9x16.ttf"))?;

    let mut viewer = WorldViewer::new(
        world,
        Colors {
            sea_low: Color::rgb(35, 45, 84),
            sea_high: Color::rgb(51, 98, 153),
            land_low: Color::rgb(33, 156, 53),
            land_high: Color::rgb(100, 230, 80),
        },
        &font,
    );
    viewer.update_buffer();

    match event::run(&mut ctx, &mut event_loop, &mut viewer) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }

    Ok(())
}
