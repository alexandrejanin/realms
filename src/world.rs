use crate::noisemap::{FalloffParameters, NoiseMap, NoiseParameters};

#[derive(Debug, Copy, Clone)]
pub struct WorldParameters {
    pub width: usize,
    pub height: usize,
    pub elevation_parameters: NoiseParameters,
    pub falloff: Option<FalloffParameters>,
    pub sea_level: f64,
}

pub struct World {
    pub seed: u64,
    pub parameters: WorldParameters,
    pub elevation: NoiseMap,
}

impl World {
    pub fn new(seed: u64, parameters: WorldParameters) -> Self {
        let elevation = Self::generate_elevation(seed, &parameters);

        Self {
            seed,
            parameters,
            elevation,
        }
    }

    pub fn generate(&mut self, seed: u64) {
        self.seed = seed;
        self.elevation = Self::generate_elevation(seed, &self.parameters);
    }

    fn generate_elevation(seed: u64, parameters: &WorldParameters) -> NoiseMap {
        match &parameters.falloff {
            Some(falloff) => NoiseMap::new_with_falloff(
                seed,
                parameters.width,
                parameters.height,
                &parameters.elevation_parameters,
                falloff,
            ),
            None => NoiseMap::new(
                seed,
                parameters.width,
                parameters.height,
                &parameters.elevation_parameters,
            ),
        }
    }
}
