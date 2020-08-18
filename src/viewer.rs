use ggez::event::KeyCode;
use ggez::graphics::{Font, TextFragment};
use ggez::input::keyboard::KeyMods;
use ggez::{
    event::{EventHandler, MouseButton},
    graphics::{self, DrawParam},
    Context, GameResult,
};
use rand::{thread_rng, RngCore};

use crate::util::{inverse_lerp, lerp};
use crate::world::{World, WorldParameters};

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn into_vec(self) -> Vec<u8> {
        vec![self.r, self.g, self.b, self.a]
    }
}

impl Into<Vec<u8>> for Color {
    fn into(self) -> Vec<u8> {
        self.into_vec()
    }
}

pub struct Colors {
    pub sea_low: Color,
    pub sea_high: Color,
    pub land_low: Color,
    pub land_high: Color,
}

pub struct WorldViewer<'f> {
    world: World,
    colors: Colors,
    buffer: Vec<u8>,
    scale: f32,
    offset: [f32; 2],
    mouse_down: bool,
    last_mouse_x: f32,
    last_mouse_y: f32,
    font: &'f Font,
    current_row: usize,
    rows: Vec<EditableRow>,
}

impl<'f> WorldViewer<'f> {
    pub fn new(world: World, colors: Colors, font: &'f Font) -> Self {
        Self {
            scale: 1000.0 / world.parameters.width as f32,
            world,
            colors,
            font,
            buffer: vec![],
            offset: [0.0, 0.0],
            mouse_down: false,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
            current_row: 0,
            rows: vec![
                EditableRow {
                    text: Box::new(|parameters| format!("sea level: {:.2}", parameters.sea_level)),
                    edit: Box::new(|parameters, action| match action {
                        EditType::Increase => parameters.sea_level += 0.1,
                        EditType::Decrease => parameters.sea_level -= 0.1,
                        _ => {}
                    }),
                },
                EditableRow {
                    text: Box::new(|parameters| {
                        format!("octaves: {}", parameters.elevation_parameters.octaves)
                    }),
                    edit: Box::new(|parameters, action| match action {
                        EditType::Increase => parameters.elevation_parameters.octaves += 1,
                        EditType::Decrease => parameters.elevation_parameters.octaves -= 1,
                        _ => {}
                    }),
                },
                EditableRow {
                    text: Box::new(|parameters| {
                        format!(
                            "persistence: {:.2}",
                            parameters.elevation_parameters.persistence,
                        )
                    }),
                    edit: Box::new(|parameters, action| match action {
                        EditType::Increase => parameters.elevation_parameters.persistence += 0.05,
                        EditType::Decrease => parameters.elevation_parameters.persistence -= 0.05,
                        _ => {}
                    }),
                },
                EditableRow {
                    text: Box::new(|parameters| {
                        format!(
                            "lacunarity: {:.2}",
                            parameters.elevation_parameters.lacunarity,
                        )
                    }),
                    edit: Box::new(|parameters, action| match action {
                        EditType::Increase => parameters.elevation_parameters.lacunarity += 0.05,
                        EditType::Decrease => parameters.elevation_parameters.lacunarity -= 0.05,
                        _ => {}
                    }),
                },
            ],
        }
    }

    pub fn update_buffer(&mut self) {
        self.buffer = (0..self.world.parameters.width * self.world.parameters.height)
            .flat_map(|i| {
                self.pixel_color(
                    i % self.world.parameters.width,
                    i / self.world.parameters.width,
                )
                .into_vec()
            })
            .collect();
    }

    fn pixel_color(&self, x: usize, y: usize) -> Color {
        let value = self.world.elevation.get(x, y);

        if value < self.world.parameters.sea_level {
            Self::interpolate_colors(
                &self.colors.sea_low,
                &self.colors.sea_high,
                inverse_lerp(
                    self.world.elevation.min,
                    self.world
                        .parameters
                        .sea_level
                        .min(self.world.elevation.max),
                    value,
                ),
            )
        } else {
            Self::interpolate_colors(
                &self.colors.land_low,
                &self.colors.land_high,
                inverse_lerp(
                    self.world
                        .parameters
                        .sea_level
                        .max(self.world.elevation.min),
                    self.world.elevation.max,
                    value,
                ),
            )
        }
    }

    fn interpolate_colors(a: &Color, b: &Color, value: f64) -> Color {
        Color::rgb(
            Self::interpolate_u8(a.r, b.r, value),
            Self::interpolate_u8(a.g, b.g, value),
            Self::interpolate_u8(a.b, b.b, value),
        )
    }

    fn interpolate_u8(a: u8, b: u8, value: f64) -> u8 {
        lerp(a as f64, b as f64, value) as u8
    }
}

impl<'f> EventHandler for WorldViewer<'f> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let mut image = graphics::Image::from_rgba8(
            ctx,
            self.world.parameters.width as u16,
            self.world.parameters.height as u16,
            &self.buffer,
        )?;
        image.set_filter(graphics::FilterMode::Nearest);

        graphics::draw(
            ctx,
            &image,
            DrawParam {
                scale: [self.scale, self.scale].into(),
                offset: self.offset.into(),
                ..Default::default()
            },
        )?;

        let text: String = self
            .rows
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let text = (row.text)(&self.world.parameters);
                if i == self.current_row {
                    format!("< {} >", text)
                } else {
                    format!("  {}  ", text)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        graphics::draw(
            ctx,
            &graphics::Text::new(TextFragment::new(text).font(*self.font)),
            DrawParam::default(),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if button == MouseButton::Left {
            self.mouse_down = true
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.mouse_down = false
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if self.mouse_down && (x != self.last_mouse_x || y != self.last_mouse_y) {
            self.offset[0] -= _dx / self.world.parameters.width as f32 / self.scale;
            self.offset[1] -= _dy / self.world.parameters.height as f32 / self.scale;
            self.last_mouse_x = x;
            self.last_mouse_y = y;
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        if y > 0.0 {
            self.scale *= 1.1;
        } else if y < 0.0 {
            self.scale /= 1.1;
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        if keycode == KeyCode::Return && !repeat {
            self.world = World::new(thread_rng().next_u64(), self.world.parameters);
            self.update_buffer();
        }

        if keycode == KeyCode::Up {
            if self.current_row == 0 {
                self.current_row = self.rows.len() - 1
            } else {
                self.current_row -= 1
            }
        }

        if keycode == KeyCode::Down {
            if self.current_row >= self.rows.len() {
                self.current_row = 0
            } else {
                self.current_row += 1;
            }
        }

        if keycode == KeyCode::Right || keycode == KeyCode::Left {
            let row = &mut self.rows[self.current_row];
            (row.edit)(
                &mut self.world.parameters,
                match keycode {
                    KeyCode::Space => EditType::Press,
                    KeyCode::Right => EditType::Increase,
                    KeyCode::Left => EditType::Decrease,
                    _ => panic!(),
                },
            );
            self.world.generate();
            self.update_buffer();
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .expect("Could not set screen coordinates");
    }
}

enum EditType {
    Increase,
    Decrease,
    Press,
}

struct EditableRow {
    text: Box<dyn Fn(&WorldParameters) -> String>,
    edit: Box<dyn FnMut(&mut WorldParameters, EditType)>,
}
